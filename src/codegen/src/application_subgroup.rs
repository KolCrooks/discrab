use proc_macro::{TokenStream};

use quote::quote;
use syn::parse_macro_input;

pub fn gen_subgroup_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    // Name of the struct that the macro is attached to
    let name = &*input.self_ty;
    let impl_ = &input.impl_token;
    let impl_generics = &mut input.generics.clone();

    let output = quote! {
        #[async_trait::async_trait]
        #input

        #impl_ #impl_generics discrab::Registerable for #name {
            fn get_reg_type(&self) -> discrab::core::abstraction::traits::RegisterableType {
                discrab::core::abstraction::traits::RegisterableType::SubCommandGroup
            }
            fn get_name(&self) -> Option<&'static str> {
                Some(Self::NAME)
            }
            fn get_description(&self) -> Option<&'static str> {
                Some(Self::DESCRIPTION)
            }
            fn get_options(&self) -> std::vec::Vec<discrab::api::ApplicationCommandOption> {
                self.get_subs().unwrap().iter().map(discrab::api::ApplicationCommandOption::from).collect()
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
