use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ImplItem, Meta, NestedMeta, Type, TypePath};
use type_check::check_type;

mod type_check;

#[proc_macro_attribute]
pub fn event_handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);
    let args = parse_macro_input!(_args as syn::AttributeArgs);
    let handler = input
        .items
        .iter()
        .find(|f| {
            if let ImplItem::Method(v) = &f {
                v.sig.ident == "handler"
            } else {
                false
            }
        })
        .expect("handler function not found");

    let handler_arg2 = if let ImplItem::Method(f) = &handler {
        if let FnArg::Typed(typ) = &f.sig.inputs[1] {
            typ.ty.clone()
        } else {
            panic!("handler function must have a second argument");
        }
    } else {
        panic!("handler field must be a bare function");
    };

    let name = &*input.self_ty;

    let event = if let NestedMeta::Meta(Meta::Path(v)) = &args[0] {
        v
    } else {
        panic!("event name must be a string")
    };

    let event_name = event.segments.last().unwrap().ident.to_string();
    let event_type = if let Type::Path(p) = &*handler_arg2 {
        p.path.segments.last().unwrap().ident.to_string()
    } else {
        panic!("handler argument must be a type")
    };

    if let Err(e) = check_type(event_name, event_type) {
        panic!(
            "Handler for {} doesn't use type {} as its second argument.",
            event.segments.last().unwrap().ident,
            e
        );
    }

    let output = quote! {
        #input
        impl discord_rs::Registerable for #name {
            fn register(&self, dispatcher: &mut discord_rs::EventDispatcher) {
                dispatcher.get_observable(#event).subscribe(&move |ctx, val| block_on(#name::handler(ctx, val)));
            }
        }
    };
    output.into()
}
