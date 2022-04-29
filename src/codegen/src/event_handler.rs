use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn gen_event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    let name = &*input.self_ty;
    let impl_ = &input.impl_token;
    let impl_generics = &mut input.generics.clone();

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

        #impl_ #impl_generics discrab::Registerable for #name {
            fn get_reg_type(&self) ->  discrab::core::abstraction::traits::RegisterableType {
                discrab::core::abstraction::traits::RegisterableType::Event
            }
        
            fn get_event_type(&self) -> Option<discrab::Events> {
                Some(#name::EVENT_TYPE)
            }
        }

        #impl_ #impl_generics discrab::RegFns for #name {
            fn reg_event(self: &std::sync::Arc<Self>, dispatcher: &mut discrab::EventDispatcher) {
                dispatcher.get_observable(#name::EVENT_TYPE).subscribe(self.clone());
            }
        }

        #impl_ #impl_generics discrab::__internal__::__InternalEventHandler<#event_type_str> for #name {
            fn handler(&self, ctx: discrab::Context, val: #event_type_str) {
                async_std::task::block_on(discrab::EventHandler::<#event_type_str>::handler(
                    self, ctx, val,
                ))
            }
        }
        
        
    };
    output.into()
}
