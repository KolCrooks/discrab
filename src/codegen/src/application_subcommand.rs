use proc_macro::{TokenStream};

use quote::quote;
use syn::parse_macro_input;

pub fn gen_sub_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
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
                discrab::core::abstraction::traits::RegisterableType::SubCommand
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
                panic!("Subcommands shouldn't be registered by the bot (add them as subs inside a command)!");
            }
        }

        #[async_trait::async_trait]
        #impl_ #impl_generics discrab::CommonHandler for #name {
            async fn handler(&self, ictx: discrab::events::InteractionCtx) {
                discrab::CommandHandler::handler(self, ictx).await
            }
        }
        
        #impl_ #impl_generics discrab::SubRegisterable for #name {}
    };
    output.into()
}
