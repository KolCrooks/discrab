use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn gen_event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    let name = &*input.self_ty;

    let output = quote! {
        #input
        impl discord_rs::Registerable for #name {
            fn register(&self, ctx: Context, dispatcher: &mut discord_rs::EventDispatcher, interaction_router: &mut discord_rs::InteractionRouter) {
                let id = async_std::task::block_on(discord_rs::InteractionRouter::get_id_or_register::<#name>(ctx.clone()));
                interaction_router.register_command(id, &move |ctx, val| async_std::task::block_on(#name::handler(ctx, val)));
            }
        }

        #[async_trait]
        impl discord_rs::CommandHandlerImpl for #name {
            async fn handler(ctx: Context, arg: Interaction) {
                <#name as CommandHandler>::handler(ctx, arg).await;
            }
        }
    };
    output.into()
}
