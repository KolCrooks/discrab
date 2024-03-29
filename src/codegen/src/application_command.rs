use proc_macro::{TokenStream};

use quote::quote;
use syn::parse_macro_input;

pub fn gen_event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    // Name of the struct that the macro is attached to
    let name = &*input.self_ty;
    let impl_ = &input.impl_token;
    let impl_generics = &mut input.generics.clone();

    let output = quote! {
        #[async_trait::async_trait]
        #input
        // Add the hook for the struct to be registerable by the interaction handler
        #impl_ #impl_generics discrab::Registerable for #name {
            fn get_reg_type(&self) -> discrab::core::abstraction::traits::RegisterableType {
                discrab::core::abstraction::traits::RegisterableType::Command
            }

            fn get_application_command_type(&self) -> Option<discrab::api::ApplicationCommandType> {
                Some(Self::COMMAND_TYPE)
            }
            
            fn get_name(&self) -> Option<&'static str> {
                Some(Self::NAME)
            }
        
            fn get_description(&self) -> Option<&'static str> {
                Some(Self::DESCRIPTION)
            }
        
            fn get_options(&self) -> std::vec::Vec<discrab::api::ApplicationCommandOption> {
                <#name as discrab::CommandHandler>::get_options()
            }
        }

        #impl_ #impl_generics discrab::RegFns for #name {
            fn reg_command(self: &std::sync::Arc<Self>, ctx: discrab::Context, router: std::sync::Arc<discrab::InteractionRouter>) {
                // Get the id of the interaction handler, or create a new one if it doesn't exist
                let id = async_std::task::block_on(discrab::InteractionRouter::get_id_or_register(ctx, self.clone()));
                // Register the handler
                router.register_command(id, self.clone());
            }
        }

        // Add the hook for the struct to convert the async handler to a sync one
        #impl_ #impl_generics discrab::__internal__::__InternalEventHandler<discrab::events::InteractionCtx> for #name {
            fn handler(&self, ctx: discrab::Context, val: discrab::events::InteractionCtx) {
                async_std::task::block_on(discrab::CommandHandler::handler(
                    self, val,
                ))
            }
        }
    };
    output.into()
}
