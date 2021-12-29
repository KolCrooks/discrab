use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ImplItem, Meta, NestedMeta, Type};

pub fn gen_event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);
    let handler = input
        .items
        .iter()
        .find(|f| {
            if let ImplItem::Method(v) = &f {
                v.sig.ident == "handler"
            } else {
                false
            }
        })
        .expect("handler function not found");

    let name = &*input.self_ty;
    let event_type_str = if let syn::PathArguments::AngleBracketed(a) = &input
        .trait_
        .as_ref()
        .unwrap()
        .1
        .segments
        .last()
        .unwrap()
        .arguments
    {
        &a.args[0]
    } else {
        panic!("Invalid event type")
    };

    let output = quote! {
        #input
        impl discord_rs::Registerable for #name {
            fn register(&self, dispatcher: &mut discord_rs::EventDispatcher) {
                dispatcher.get_observable(#name::EVENT_TYPE, "#event_type_str").subscribe(&move |ctx, val| async_std::task::block_on(#name::handler(ctx, val)));
            }
        }
    };
    output.into()
}
