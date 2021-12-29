use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FnArg, ImplItem, Meta, NestedMeta, Type};
mod application_command;
mod event_handler;

#[proc_macro_attribute]
pub fn event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    event_handler::gen_event_handler(_args, input)
}

#[proc_macro_attribute]
pub fn application_command(_args: TokenStream, input: TokenStream) -> TokenStream {
    application_command::gen_event_handler(_args, input)
}

#[proc_macro_derive(CommandArg)]
pub fn command_arg_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl CommandArg for #name {
        }
    };
    gen.into()
}
