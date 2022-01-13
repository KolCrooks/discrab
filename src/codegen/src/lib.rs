use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
mod application_command;
mod event_handler;

#[proc_macro_attribute]
/// Generates additional code needed to register an EventHandler
pub fn event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    event_handler::gen_event_handler(_args, input)
}

#[proc_macro_attribute]
/// Generates additional code needed to register a CommandHandler
pub fn command(_args: TokenStream, input: TokenStream) -> TokenStream {
    application_command::gen_event_handler(_args, input)
}

#[proc_macro_derive(CommandArg)]
/// Implements CommandArg for a struct
pub fn command_arg_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl CommandArg for #name {
        }
    };
    gen.into()
}
