use http::methods::{POST, PUT};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Pat;
use syn::{FnArg, Ident, ItemFn, PatType, ReturnType, Type};

/// [`function_to_route`] takes the parsed attribute, `path_lit`, and the original
/// function, then reconstructs a new function in it's place that returns a
/// [`Route`](http::route::Route).
pub(crate) fn function_to_route(
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
    let mut had_body_arg = false;
    let fn_attrs = item_fn.attrs;
    let fn_vis = item_fn.vis;
    let mut fn_sig = item_fn.sig;
    let fn_block = item_fn.block;
    let original_fn_ident = fn_sig.ident.clone();
    let path_str_value = path_lit.value();
    let request_type_ident: Type = syn::parse_str("http::request::Request").unwrap();
    let internal_fn_ident = generate_unique_internal_fn_name(method_str, &original_fn_ident);
    let path_param_names = extract_path_parameters(&path_str_value, &original_fn_ident);

    // extract original inputs, output, and generics for the internal function, these
    // are cloned these because `fn_sig` will be mutated for the public function.
    let original_inputs = fn_sig.inputs.clone();
    let original_output = fn_sig.output.clone();
    let original_generics = &fn_sig.generics;

    // iterate over the original function's arguments to generate extraction logic
    let mut original_fn_call_args = Vec::new();
    let mut extracted_arg_prelude = Vec::new();
    for arg in &original_inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            let param_ident = if let Pat::Ident(pat_ident) = &**pat {
                pat_ident.ident.clone()
            } else {
                return syn::Error::new_spanned(
                    pat,
                    "Only identifier patterns are supported for function arguments in HTTP handlers",
                ).to_compile_error().into();
            };

            if path_param_names.contains(&param_ident) {
                extracted_arg_prelude.push(quote!{
                    let #pat: #ty = req.path_params
                        .get(stringify!(#param_ident))
                        .expect(&format!("Missing path parameter: {}", stringify!(#param_ident)))
                        .parse()
                        .expect(&format!("Invalid path parameter type for {}: expected {}", stringify!(#param_ident), stringify!(#ty)));
                });
            } else if (method_str == POST || method_str == PUT) && !had_body_arg {
                extracted_arg_prelude.push(quote! {
                    let #pat: #ty = req.body_as_string()
                        .expect("Failed to get request body as string")
                        .parse()
                        .expect("Failed to parse request body into expected type.");
                });
                had_body_arg = true;
            } else {
                // TODO: fix wrong query string parameter type (i.e.: /squared?number=AAA will crash the server)
                extracted_arg_prelude.push(quote!{
                    let #pat: #ty = req.query_param(stringify!(#param_ident))
                        .and_then(|s| s.parse().ok())
                        .expect(&format!("Missing or invalid query parameter: {}", stringify!(#param_ident)));
                });
            }
            original_fn_call_args.push(quote! { #param_ident });
        } else {
            return syn::Error::new_spanned(
                arg,
                "Receiver arguments (like &self) are not supported in HTTP handlers.",
            )
            .to_compile_error()
            .into();
        }
    }

    // reconstruct the original function as an internal helper
    let original_fn_impl = quote! {
        #(#fn_attrs)*
        #fn_vis fn #internal_fn_ident #original_generics (#original_inputs) #original_output #fn_block
    };

    // prepare the public function's signature, note `fn_sig` is *mutated* to become the public signature.
    fn_sig.inputs = syn::parse_quote! {};
    fn_sig.output = syn::parse_quote! { -> http::route::Route };
    let method_ident = syn::Ident::new(&method_str.to_lowercase(), Span::call_site());
    let handler_closure = quote! {
        Box::new(
            |req: #request_type_ident| -> http::response::Response {
                #(#extracted_arg_prelude)*
                let content = #internal_fn_ident(#(#original_fn_call_args),*);
                http::response::Response::ok(&content, #is_raw)
            }
        ) as http::route::RouteHandler
    };
    let expanded = quote! {
        #original_fn_impl

        #(#fn_attrs)*
        #fn_vis #fn_sig {
            http::route::Route::#method_ident(#path_str_value, #handler_closure)
        }
    };

    // return the syntax tree as a TokenStream
    expanded.into()
}

/// [`validate_return_type`] generates a custom error message for the http attributes.
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

/// [`generate_unique_internal_fn_name`] creates a unique name for the generated internal
/// function that will be maintained as a result of using any of the http attributes.
fn generate_unique_internal_fn_name(method_str: &str, original_fn_ident: &Ident) -> Ident {
    Ident::new(
        &format!(
            "__http_{}_handler_orig_{}",
            method_str.to_lowercase(),
            original_fn_ident
        ),
        Span::call_site(),
    )
}

/// [`extract_path_parameters`] will return a collection of [`Ident`] that are surrounded
/// by `{` and `}`.
fn extract_path_parameters(path_str_value: &String, original_fn_ident: &Ident) -> Vec<Ident> {
    path_str_value
        .split('/')
        .filter(|segment| segment.starts_with('{') && segment.ends_with('}'))
        .map(|segment| Ident::new(&segment[1..segment.len() - 1], original_fn_ident.span()))
        .collect()
}
