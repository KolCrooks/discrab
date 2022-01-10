use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem};

pub fn gen_event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

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
        impl<'a> discord_rs::Registerable<'a> for #name {
            fn register(
                &'a self,
                ctx: discord_rs::Context,
                dispatcher: &mut discord_rs::EventDispatcher<'a>,
                _: &mut discord_rs::InteractionRouter<'a>,
            ) {
                dispatcher.get_observable(#name::EVENT_TYPE, stringify!(#event_type_str)).subscribe(self);
            }
        }

        impl discord_rs::EventHandlerImpl<#event_type_str> for #name {
            fn handler(&self, ctx: discord_rs::Context, val: #event_type_str) {
                async_std::task::block_on(discord_rs::EventHandler::<#event_type_str>::handler(
                    self, ctx, val,
                ))
            }
        }
    };
    output.into()
}
