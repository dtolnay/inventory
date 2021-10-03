extern crate proc_macro;

use std::collections::hash_map;
use std::hash::Hasher;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{bracketed, parse_macro_input, Path, Token, Type};

struct Input {
    krate: Option<Path>,
    collect_type: Option<Type>,
    expr: TokenStream,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut krate = None;
        let mut collect_type = None;

        // #![crate = gflags, type = Flags]
        if input.peek(Token![#]) && input.peek2(Token![!]) {
            input.parse::<Token![#]>()?;
            input.parse::<Token![!]>()?;
            let content;
            bracketed!(content in input);
            let mut is_comma_sep = true;
            if content.peek(Token![crate]) {
                content.parse::<Token![crate]>()?;
                content.parse::<Token![=]>()?;
                krate = Some(content.parse()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                    is_comma_sep = true;
                } else {
                    is_comma_sep = false;
                }
            }
            if is_comma_sep && content.peek(Token![type]) {
                content.parse::<Token![type]>()?;
                content.parse::<Token![=]>()?;
                collect_type = Some(content.parse()?);
            }
        }

        Ok(Input {
            krate,
            collect_type,
            expr: input.parse()?,
        })
    }
}

#[proc_macro]
#[doc(hidden)]
pub fn submit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let expr = input.expr;
    let init = Ident::new(&format!("__init{}", hash(&expr)), Span::call_site());
    let prefix = match input.krate {
        Some(krate) => quote!(#krate::),
        None => quote!(),
    };

    let expanded = if let Some(collect_type) = input.collect_type {
        quote! {
            #[allow(non_upper_case_globals)]
            #[#prefix ::inventory::ctor]
            unsafe fn #init() {
                use #prefix ::inventory::{Collect, StaticNode, Node, submit_raw};

                static mut NODE: StaticNode<#collect_type> = unsafe { StaticNode::new() };

                let node_ptr = NODE.get();
                let val: #collect_type = #expr;
                ::core::ptr::write(node_ptr, Node::new(val));

                submit_raw(node_ptr);
            }
        }
    } else {
        quote! {
            #[allow(non_upper_case_globals)]
            #[#prefix ::inventory::ctor]
            fn #init() {
                // TODO: once existential type is stable, store the caller's
                // expression into a static and string those statics together into
                // an intrusive linked list without needing allocation.
                //
                //     existential type This;
                //
                //     static mut VALUE: Option<inventory::Node<This>> = None;
                //
                //     fn value() -> This {
                //         #expr
                //     }
                //
                //     unsafe {
                //         VALUE = Some(inventory::Node {
                //             value: value(),
                //             next: None,
                //         });
                //         inventory::submit(VALUE.as_mut().unwrap());
                //     }

                #prefix ::inventory::submit({ #expr });
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn hash(input: &TokenStream) -> u64 {
    let mut hasher = hash_map::DefaultHasher::new();
    hasher.write(input.to_string().as_bytes());
    hasher.finish()
}
