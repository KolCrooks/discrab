use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ImplItem, Meta, NestedMeta, Type};

pub fn gen_event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);
    let args = parse_macro_input!(_args as syn::AttributeArgs);

    let name = &*input.self_ty;

    let event = if let NestedMeta::Meta(Meta::Path(v)) = &args[0] {
        v
    } else {
        panic!("event name must be a string")
    };

    let output = quote! {
        #input
        impl discord_rs::RegisterableImpl<discord_rs::command_args::Interaction> for #name {
            const EVENT_TYPE: discord_rs::Events = discord_rs::Events::InteractionCreate;
        }
        impl discord_rs::Registerable for #name {
            fn register(&self, dispatcher: &mut discord_rs::EventDispatcher) {
                dispatcher
                    .get_observable(discord_rs::Events::InteractionCreate, "Interaction")
                    .subscribe(&move |ctx, val| async_std::task::block_on(<#name as discord_rs::ApplicationCommandHandler>::handler(ctx, val)));
            }
        }

        #[async_trait]
        impl discord_rs::EventHandler<discord_rs::command_args::Interaction> for #name {
            async fn handler(ctx: Context, arg: Interaction) {
                <#name as ApplicationCommandHandler>::handler(ctx, arg).await;
            }
        }
    };
    output.into()
}
