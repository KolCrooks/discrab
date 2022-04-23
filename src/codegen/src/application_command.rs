use proc_macro::{TokenStream};
use proc_macro2::Span;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, Lifetime, punctuated::Punctuated};

pub fn gen_event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    // Name of the struct that the macro is attached to
    let name = &*input.self_ty;


    let output = quote! {
        #[async_trait::async_trait]
        #input
        // Add the hook for the struct to be registerable by the interaction handler
        impl<'a> discrab::Registerable<'a> for #name {
            fn register(
                &'a self,
                ctx: discrab::Context,
                _: &mut discrab::EventDispatcher<'a>,
                interaction_router: &mut discrab::InteractionRouter<'a>,
            ){
                // Get the id of the interaction handler, or create a new one if it doesn't exist
                let id = async_std::task::block_on(discrab::InteractionRouter::get_id_or_register::<#name>(ctx, self));
                // Register the handler
                interaction_router.register_command(id, self);
            }

            /// @returns the applications type, name, and description
            fn get_info(&self) -> (discrab::core::abstraction::abstraction_traits::RegisterableType, discrab::api::ApplicationCommandType, &'static str, Option<&'static str>) {
                (discrab::core::abstraction::abstraction_traits::RegisterableType::Command, Self::COMMAND_TYPE, Self::COMMAND_NAME, Some(Self::COMMAND_DESCRIPTION))
            }

            fn get_options(&self) -> Vec<ApplicationCommandOption> {
                <#name as discrab::CommandHandler<'_>>::get_options()
            }
        }

        // Add the hook for the struct to convert the async handler to a sync one
        impl discrab::__internal__::InternalEventHandler<discrab::events::InteractionCtx> for #name {
            fn handler(&self, ctx: discrab::Context, val: discrab::events::InteractionCtx) {
                async_std::task::block_on(discrab::CommandHandler::handler(
                    self, val,
                ))
            }
        }
    };
    output.into()
}
