use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn gen_event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    let name = &*input.self_ty;

    let output = quote! {
        #input
        impl<'a> discord_rs::Registerable<'a> for #name {
            fn register(
                &'a self,
                ctx: discord_rs::Context,
                _: &mut discord_rs::EventDispatcher<'a>,
                interaction_router: &mut discord_rs::InteractionRouter<'a>,
            ){
                let id = async_std::task::block_on(discord_rs::InteractionRouter::get_id_or_register::<#name>(ctx));
                interaction_router.register_command(id, self);
            }
        }

        impl discord_rs::EventHandlerImpl<discord_rs::command_args::InteractionCreate> for #name {
            fn handler(&self, ctx: discord_rs::Context, val: discord_rs::command_args::InteractionCreate) {
                async_std::task::block_on(discord_rs::CommandHandler::handler(
                    self, ctx, val,
                ))
            }
        }
    };
    output.into()
}
