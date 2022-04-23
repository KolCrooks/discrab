use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

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
        #[async_trait::async_trait]
        #input
        impl<'a> discrab::Registerable<'a> for #name {
            fn register(
                &'a self,
                ctx: discrab::Context,
                dispatcher: &mut discrab::EventDispatcher<'a>,
                _: &mut discrab::InteractionRouter<'a>,
            ) {
                dispatcher.get_observable(#name::EVENT_TYPE, stringify!(#event_type_str)).subscribe(self);
            }
            fn get_options(&self) -> Vec<discrab::api::ApplicationCommandOption> {
                vec![]
            }
        }

        impl discrab::__internal__::InternalEventHandler<#event_type_str> for #name {
            fn handler(&self, ctx: discrab::Context, val: #event_type_str) {
                async_std::task::block_on(discrab::EventHandler::<#event_type_str>::handler(
                    self, ctx, val,
                ))
            }
        }
        
        
    };
    output.into()
}
