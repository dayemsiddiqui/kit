//! Injectable attribute macro for the Kit framework
//!
//! Provides the `#[injectable]` attribute macro that auto-registers
//! concrete types as singletons in the App container.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Implements the `#[injectable]` attribute macro
///
/// This macro automatically:
/// 1. Derives `Default` and `Clone` for the type
/// 2. Registers the type as a singleton in the App container at startup
///
/// # Example
///
/// ```rust,ignore
/// use kit::injectable;
///
/// #[injectable]
/// pub struct AppState {
///     pub counter: u32,
/// }
///
/// // Automatically registered at startup
/// // Resolve via:
/// let state: AppState = App::get().unwrap();
/// ```
pub fn injectable_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();
    let vis = &input.vis;
    let attrs = &input.attrs;
    let generics = &input.generics;

    let expanded = match &input.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;

            quote! {
                #(#attrs)*
                #[derive(Default, Clone)]
                #vis struct #name #generics #fields

                // Auto-register this type as a singleton at startup
                ::kit::inventory::submit! {
                    ::kit::container::provider::SingletonEntry {
                        register: || {
                            ::kit::App::singleton(<#name as ::std::default::Default>::default());
                        },
                        name: #name_str,
                    }
                }
            }
        }
        _ => {
            syn::Error::new_spanned(&input, "injectable can only be used on structs")
                .to_compile_error()
        }
    };

    TokenStream::from(expanded)
}
