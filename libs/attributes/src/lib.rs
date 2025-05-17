use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ReturnType, Type, parse_macro_input};

fn extract_return_type(a: ReturnType) -> Box<Type> {
    match a {
        ReturnType::Type(_, p) => p,
        _ => panic!("Not supported on functions without return types!"),
    }
}

#[proc_macro_attribute]
pub fn http_get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block.stmts.last();
    let fn_return_type = &input_fn.sig.output;
    let route_return_type = quote! {http::route::Route};
    let binding = attr.to_string();
    let path_string = binding.trim_matches('"');
    let route_get = quote! {http::route::Route::get(#path_string, #fn_body)};

    extract_return_type(fn_return_type.clone());

    let expanded = quote! {
        pub fn #fn_name() -> #route_return_type {
            #route_get
        }
    };

    expanded.into()
}
