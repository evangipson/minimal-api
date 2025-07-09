//! # [`attributes`](crate)
//! The [`attributes`](crate) crate contains macros to make it easy to generate a
//! [`Route`](http::route::Route) just by adding one of the attributes it provides.
//!
//! It provides helpful `#[proc_macro_attribute]` macros to enable functions to
//! return a [`Route`](http::route::Route) for various request types, such as:
//! - [`http_get`](macro@http_get)
//! - [`http_raw_get`](macro@http_raw_get)
//! - [`http_post`](macro@http_post)
//! - [`http_raw_post`](macro@http_raw_post)

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{FnArg, ItemFn, PatType, ReturnType, Type, parse_macro_input};

const REQUEST_TYPE_PLACEHOLDER: &str = "http::request::Request";
const GET_METHOD: &str = "GET";
const POST_METHOD: &str = "POST";

/// `validate_return_type` generates a custom error message for the macros.
fn validate_return_type(item_fn: &ItemFn, method: &str) -> Result<(), TokenStream> {
    let original_return_type = match &item_fn.sig.output {
        ReturnType::Type(_, ty) => ty,
        _ => {
            return Err(syn::Error::new_spanned(
                &item_fn.sig.output,
                format!("Functions marked with http_{} must have an explicit return type (e.g., `-> String`).", method),
            )
            .to_compile_error()
            .into());
        }
    };

    let expected_return_type: Type = syn::parse_quote! { String };

    // Compare the concrete types (ignoring potential path/query params which aren't part of the type)
    if quote! {#original_return_type}.to_string() != quote! {#expected_return_type}.to_string() {
        return Err(syn::Error::new_spanned(
            original_return_type,
            format!(
                "http_{} functions must return `String`, but found `{}`",
                method,
                quote! {#original_return_type}
            ),
        )
        .to_compile_error()
        .into());
    }
    Ok(())
}

/// [`transform_function_to_route_handler`] takes the parsed attribute
/// (path) and the original function, then reconstructs the new function.
fn transform_function_to_route_handler(
    path_lit: syn::LitStr,
    item_fn: ItemFn,
    method_str: &str,
    is_raw: bool,
) -> TokenStream {
    // validate the return type
    if let Err(e) = validate_return_type(&item_fn, method_str) {
        return e;
    }

    // get information from the original function to create the route handler
    let fn_attrs = item_fn.attrs;
    let fn_vis = item_fn.vis;
    let mut fn_sig = item_fn.sig;
    let fn_block = item_fn.block;
    let original_fn_ident = fn_sig.ident.clone();

    // generate a unique name for the internal, original function's logic
    let internal_fn_ident = syn::Ident::new(
        &format!(
            "__http_{}_handler_orig_{}",
            method_str.to_lowercase(),
            original_fn_ident
        ),
        Span::call_site(),
    );

    let mut extracted_args = quote! {};
    let mut original_func_call_args = quote! {};
    let mut had_body_arg = false;

    let request_type_ident: Type =
        syn::parse_str(REQUEST_TYPE_PLACEHOLDER).expect("Invalid RequestType placeholder");

    // extract original inputs, output, and generics for the internal function, these
    // are cloned these because `fn_sig` will be mutated for the public function.
    let original_inputs = fn_sig.inputs.clone();
    let original_output = fn_sig.output.clone();
    let original_generics = &fn_sig.generics;

    // iterate over the original function's arguments to generate extraction logic
    for arg in original_inputs.iter() {
        match arg {
            FnArg::Receiver(_) => {
                return syn::Error::new_spanned(
                    arg,
                    "Receiver arguments (e.g., `&self`, `self`) are not supported in http_get/post functions.",
                )
                .to_compile_error()
                .into();
            }
            FnArg::Typed(PatType { pat, ty, .. }) => {
                if method_str == "POST" && !had_body_arg {
                    extracted_args.extend(quote! {
                        let #pat: #ty = req.body_as_string()
                            .expect("Failed to get request body as string")
                            .parse()
                            .expect("Failed to parse request body into expected type.");
                    });
                    had_body_arg = true;
                } else {
                    extracted_args.extend(quote! {
                        let #pat: #ty = req.query_param(stringify!(#pat))
                            .and_then(|s| s.parse().ok())
                            .expect(
                                &format!("Missing or invalid query parameter: {}",
                                stringify!(#pat)
                            ));
                    });
                }
                original_func_call_args.extend(quote! { #pat, });
            }
        }
    }

    // reconstruct the original function as an internal helper
    let original_fn_impl = quote! {
        #(#fn_attrs)* // re-apply original attributes
        #fn_vis fn #internal_fn_ident #original_generics (#original_inputs) #original_output #fn_block
    };

    // prepare the public function's signature- `fn_sig` is *mutated* to become the public signature.
    fn_sig.inputs = syn::parse_quote! {};
    fn_sig.output = syn::parse_quote! { -> http::route::Route };

    let path_str_value = path_lit.value();
    let method_enum_ident = syn::Ident::new(&method_str.to_lowercase(), Span::call_site());

    let handler_closure = quote! {
        Box::new(
            |req: #request_type_ident| -> http::response::Response {
                #extracted_args
                let content = #internal_fn_ident(#original_func_call_args);
                http::response::Response::ok(&content, #is_raw)
            }
        ) as http::route::RouteHandler
    };

    // create a syntax tree for the route handler and now internal private func
    let expanded = quote! {
        #original_fn_impl

        #(#fn_attrs)*
        #fn_vis #fn_sig {
            http::route::Route::#method_enum_ident(
                #path_str_value,
                #handler_closure
            )
        }
    };

    // return the syntax tree as a TokenStream
    expanded.into()
}

#[doc = r#"
# http_get
The [`http_get`](macro@http_get) attribute modifies the function that uses it inline
to return a [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will ensure the [`Response`](http::response::Response) returns the
matched [`Route`](http::route::Route) with the following shape:
```json
{
    "content": "...",
    "status": "...",
    "time": "...",
    "header": "...",
}
```

To get a response that is only what would be returned in "content", use the
[`http_raw_get`](macro@http_raw_get) attribute instead.

# Examples
## Basic `GET` route
[`http_get`](macro@http_get) can be used to create a basic `GET` route which returns
a simple [`String`]:
```rust
use attributes::http_get;

// this route listens for a request on the "/" path, and returns "hello".
#[http_get("/")]
fn some_request() -> String {
    "hello".to_string()
}
```

## `GET` route with query parameters
[`http_get`](macro@http_get) can also be used to get values from query parameters,
and perform actions on them before returning a [`String`] result:
```rust
use attributes::http_get;

// this will listen for a "number" query parameter, such as "/squared?number=2",
// which would return "4".
#[http_get("/squared")]
fn get_squared_query_parameter(number: i32) -> String {
    let squared_result = number * number;
    squared_result.to_string()
}
```
"#]
#[proc_macro_attribute]
pub fn http_get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(attr as syn::LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);
    transform_function_to_route_handler(path_lit, input_fn, GET_METHOD, false)
}

#[doc = r#"
# http_raw_get
The [`http_raw_get`](macro@http_raw_get) attribute modifies the function that uses it inline
to return a [`Route`](http::route::Route), as long as the function returns a [`String`].

This attribute will always return a [`Response`](http::response::Response) with the shape of
the [`Route`](http::route::Route) result.

To get a response that has more information, like "status" and "time", use the
[`http_get`](macro@http_get) attribute instead.

# Examples
## Basic `GET` route
[`http_raw_get`](macro@http_raw_get) can be used to create a basic `GET` route which returns
a simple [`String`]:
```rust
use attributes::http_raw_get;

// this route listens for a request on the "/" path, and returns "hello".
#[http_raw_get("/")]
fn some_request() -> String {
    "hello".to_string()
}
```

## `GET` route with query parameters
[`http_raw_get`](macro@http_raw_get) can also be used to get values from query parameters,
and perform actions on them before returning a [`String`] result:
```rust
use attributes::http_raw_get;

// this will listen for a "number" query parameter, such as "/squared?number=2",
// which would return "4".
#[http_raw_get("/squared")]
fn get_squared_query_parameter(number: i32) -> String {
    let squared_result = number * number;
    squared_result.to_string()
}
```
"#]
#[proc_macro_attribute]
pub fn http_raw_get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(attr as syn::LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);
    transform_function_to_route_handler(path_lit, input_fn, GET_METHOD, true)
}

#[proc_macro_attribute]
pub fn http_post(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(attr as syn::LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);
    transform_function_to_route_handler(path_lit, input_fn, POST_METHOD, false)
}

#[proc_macro_attribute]
pub fn http_raw_post(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(attr as syn::LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);
    transform_function_to_route_handler(path_lit, input_fn, POST_METHOD, true)
}
