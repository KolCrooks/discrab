use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

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
                let id = async_std::task::block_on(discrab::InteractionRouter::get_id_or_register::<#name>(ctx));
                // Register the handler
                interaction_router.register_command(id, self);
            }
        }

        // Add the hook for the struct to convert the async handler to a sync one
        impl discrab::__internal__::InternalEventHandler<discrab::events::InteractionCreate> for #name {
            fn handler(&self, ctx: discrab::Context, val: discrab::events::InteractionCreate) {
                async_std::task::block_on(discrab::CommandHandler::handler(
                    self, ctx, val,
                ))
            }
        }
    };
    output.into()
}
