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
        impl<'a> discord_rs::Registerable<'a> for #name {
            fn register(
                &'a self,
                ctx: discord_rs::Context,
                _: &mut discord_rs::EventDispatcher<'a>,
                interaction_router: &mut discord_rs::InteractionRouter<'a>,
            ){
                // Get the id of the interaction handler, or create a new one if it doesn't exist
                let id = async_std::task::block_on(discord_rs::InteractionRouter::get_id_or_register::<#name>(ctx));
                // Register the handler
                interaction_router.register_command(id, self);
            }
        }

        // Add the hook for the struct to convert the async handler to a sync one
        impl discord_rs::__internal__::InternalEventHandler<discord_rs::command_args::InteractionCreate> for #name {
            fn handler(&self, ctx: discord_rs::Context, val: discord_rs::command_args::InteractionCreate) {
                async_std::task::block_on(discord_rs::CommandHandler::handler(
                    self, ctx, val,
                ))
            }
        }
    };
    output.into()
}
