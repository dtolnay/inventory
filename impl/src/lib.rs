extern crate proc_macro;

use std::collections::hash_map;
use std::hash::Hasher;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
#[doc(hidden)]
pub fn submit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expr = parse_macro_input!(input as TokenStream);
    let init = Ident::new(&format!("__init{}", hash(&expr)), Span::call_site());

    let expanded = quote! {
        #[inventory::ctor]
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

            inventory::submit({ #expr });
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn hash(input: &TokenStream) -> u64 {
    let mut hasher = hash_map::DefaultHasher::new();
    hasher.write(input.to_string().as_bytes());
    hasher.finish()
}
