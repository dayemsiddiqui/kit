//! Handler attribute macro implementation
//!
//! Transforms controller functions to automatically extract typed parameters
//! from HTTP requests.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Type};

/// Implementation of the `#[handler]` attribute macro
///
/// Transforms:
/// ```rust,ignore
/// #[handler]
/// pub async fn store(form: CreateUserRequest) -> Response { ... }
/// ```
///
/// Into:
/// ```rust,ignore
/// pub async fn store(__kit_req: kit_rs::Request) -> kit_rs::Response {
///     let form = match <CreateUserRequest as kit_rs::FromRequest>::from_request(__kit_req).await {
///         Ok(v) => v,
///         Err(e) => return Err(e.into()),
///     };
///     // original body
/// }
/// ```
pub fn handler_impl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_vis = &input_fn.vis;
    let fn_name = &input_fn.sig.ident;
    let fn_generics = &input_fn.sig.generics;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    // Check if async
    let is_async = input_fn.sig.asyncness.is_some();
    let async_token = if is_async {
        quote! { async }
    } else {
        quote! {}
    };

    // Get the first parameter
    let first_param = input_fn.sig.inputs.first();

    match first_param {
        Some(FnArg::Typed(pat_type)) => {
            let param_pat = &pat_type.pat;
            let param_type = &pat_type.ty;

            // Check if the parameter type is Request (no extraction needed)
            let is_request_type = is_request_type(param_type);

            if is_request_type {
                // If the param is already Request, just pass through
                let output = quote! {
                    #(#fn_attrs)*
                    #fn_vis #async_token fn #fn_name #fn_generics(__kit_req: kit_rs::Request) #fn_output {
                        let #param_pat = __kit_req;
                        #fn_block
                    }
                };
                output.into()
            } else {
                // Extract the type using FromRequest
                let output = quote! {
                    #(#fn_attrs)*
                    #fn_vis #async_token fn #fn_name #fn_generics(__kit_req: kit_rs::Request) #fn_output {
                        let #param_pat: #param_type = match <#param_type as kit_rs::FromRequest>::from_request(__kit_req).await {
                            Ok(v) => v,
                            Err(e) => return Err(e.into()),
                        };
                        #fn_block
                    }
                };
                output.into()
            }
        }
        Some(FnArg::Receiver(_)) => {
            // Self receiver - not supported
            syn::Error::new_spanned(
                first_param,
                "#[handler] does not support methods with self receiver",
            )
            .to_compile_error()
            .into()
        }
        None => {
            // No parameters - generate a handler that takes Request but ignores it
            let output = quote! {
                #(#fn_attrs)*
                #fn_vis #async_token fn #fn_name #fn_generics(_: kit_rs::Request) #fn_output {
                    #fn_block
                }
            };
            output.into()
        }
    }
}

/// Check if the type is `Request` or `kit_rs::Request`
fn is_request_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            let segments = &type_path.path.segments;
            if segments.len() == 1 {
                return segments[0].ident == "Request";
            }
            if segments.len() == 2 {
                return segments[0].ident == "kit_rs" && segments[1].ident == "Request";
            }
            false
        }
        _ => false,
    }
}
