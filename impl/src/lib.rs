extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{bracketed, parse_macro_input, Path, Token};

struct Input {
    krate: Option<Path>,
    expr: TokenStream,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Input {
            krate: {
                // #![crate = gflags]
                if input.peek(Token![#]) && input.peek2(Token![!]) {
                    input.parse::<Token![#]>()?;
                    input.parse::<Token![!]>()?;
                    let content;
                    bracketed!(content in input);
                    content.parse::<Token![crate]>()?;
                    content.parse::<Token![=]>()?;
                    let krate = content.parse()?;
                    Some(krate)
                } else {
                    None
                }
            },
            expr: input.parse()?,
        })
    }
}

#[proc_macro]
#[doc(hidden)]
pub fn submit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let expr = input.expr;
    let prefix = match input.krate {
        Some(krate) => quote!(#krate::),
        None => quote!(),
    };

    let expanded = quote! {
        const _: () = {
            #[allow(non_upper_case_globals)]
            #[#prefix inventory::ctor]
            fn __init() {
                // TODO: once existential type is stable, store the caller's
                // expression into a static and string those statics together
                // into an intrusive linked list without needing allocation.
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

                #prefix inventory::submit({ #expr });
            }
        };
    };

    proc_macro::TokenStream::from(expanded)
}
