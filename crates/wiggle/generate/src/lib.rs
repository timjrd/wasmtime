mod codegen_settings;
pub mod config;
mod funcs;
mod lifetimes;
mod module_trait;
mod names;
mod types;
pub mod wasmtime;

use heck::ToShoutySnakeCase;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub use codegen_settings::CodegenSettings;
pub use config::{Config, WasmtimeConfig};
pub use funcs::define_func;
pub use module_trait::define_module_trait;
pub use names::Names;
pub use types::define_datatype;

pub fn generate(doc: &witx::Document, names: &Names, settings: &CodegenSettings) -> TokenStream {
    let types = doc.typenames().map(|t| define_datatype(&names, &t));
    let error_types = settings.errors.iter().map(|errtype| {
        let abi_typename = names.type_(&errtype.name);
        quote! {
            impl std::fmt::Display for #abi_typename {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for #abi_typename {}

            impl From<#abi_typename> for wiggle::Error<#abi_typename> {
                fn from(t: #abi_typename) -> wiggle::Error<#abi_typename> {
                    wiggle::Error::new(t)
                }
            }
        }
    });

    let constants = doc.constants().map(|c| {
        let name = quote::format_ident!(
            "{}_{}",
            c.ty.as_str().to_shouty_snake_case(),
            c.name.as_str().to_shouty_snake_case()
        );
        let ty = names.type_(&c.ty);
        let value = Literal::u64_unsuffixed(c.value);
        quote! {
            pub const #name: #ty = #value;
        }
    });

    let modules = doc.modules().map(|module| {
        let modname = names.module(&module.name);
        let fs = module
            .funcs()
            .map(|f| define_func(&names, &module, &f, &settings));
        let modtrait = define_module_trait(&names, &module, &settings);
        let wasmtime = if settings.wasmtime {
            crate::wasmtime::link_module(&module, &names, None, &settings)
        } else {
            quote! {}
        };
        quote!(
            pub mod #modname {
                use super::types::*;
                #(#fs)*

                #modtrait

                #wasmtime
            }
        )
    });

    quote!(
        pub mod types {
            use std::convert::TryFrom;

            #(#types)*
            #(#error_types)*
            #(#constants)*
        }
        #(#modules)*
    )
}

pub fn generate_metadata(doc: &witx::Document, names: &Names) -> TokenStream {
    let rt = names.runtime_mod();
    let doc_text = &format!("{}", doc);
    quote! {
        pub mod metadata {
            pub const DOC_TEXT: &str = #doc_text;
            pub fn document() -> #rt::witx::Document {
                #rt::witx::parse(DOC_TEXT).unwrap()
            }
        }
    }
}
