use std::hash::{DefaultHasher, Hash, Hasher};

use proc_macro::TokenStream;
use quote::quote;
use syn::{meta::ParseNestedMeta, parse_macro_input, Result};

struct HandlerArgs {
    priority: syn::LitInt,
    router: Option<syn::Expr>,
}

impl Default for HandlerArgs {
    fn default() -> Self {
        Self {
            priority: syn::LitInt::new("0", proc_macro2::Span::call_site()),
            router: None,
        }
    }
}

impl HandlerArgs {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if let Some(ident) = meta.path.get_ident() {
            match ident.to_string().as_str() {
                "router" => {
                    self.router = Some(meta.value()?.parse()?);
                    Ok(())
                }
                "priority" => {
                    self.priority = meta.value()?.parse()?;
                    Ok(())
                }
                _ => Err(meta.error("msg")),
            }
        } else {
            Err(meta.error("msg"))
        }
    }

    fn is_empty(&self) -> bool {
        self.router.is_none()
    }
}

fn get_router(handler_args: &HandlerArgs) -> syn::Expr {
    if let Some(router) = &handler_args.router {
        router.clone()
    } else {
        syn::parse(quote! { "AnyRouter::default()" }.into()).unwrap()
    }
}

fn get_hash_id(ident: &syn::Ident) -> String {
    let mut hasher = DefaultHasher::new();
    ident.hash(&mut hasher);
    hasher.finish().to_string()
}

fn extract_fn_name_ident(item: &syn::Ident, hash_id: &str) -> syn::Ident {
    let mut fn_name = String::from("__");
    fn_name.push_str(&item.to_string());
    fn_name.extend("_".chars().chain(hash_id.chars()));
    syn::Ident::new(&fn_name, item.span())
}

#[proc_macro_attribute]
pub fn register(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    let mut attrs = HandlerArgs::default();
    let parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(attr with parser);

    let vis = &input.vis;
    match vis {
        syn::Visibility::Public(_) => {}
        _ => {
            return TokenStream::from(
                quote! { compile_error!("Only public functions can be registered"); },
            )
        }
    }

    let origin_ident = &input.sig.ident;
    let hash_id = get_hash_id(origin_ident);
    let fn_name_ident = extract_fn_name_ident(origin_ident, &hash_id);

    let fn_args = &input.sig.inputs;
    let fn_body = &input.block;

    let router = get_router(&attrs);
    let priority = &attrs.priority;

    if attrs.is_empty() {
        return TokenStream::from(
            quote! { compile_error!("Missing `#[register(router = \"...\")]` attribute"); },
        );
    };

    let expanded = quote! {
        use std::sync::*;
        use std::cell::*;
        use aionbot::prelude::*;

        pub fn #fn_name_ident(#fn_args) -> HandlerCallback {
            Box::pin(async move { #fn_body })
        }

        pub fn #origin_ident() -> Entry {
            Entry {
                id: #hash_id,
                priority: #priority,
                router: Arc::new(Box::new(#router)),
                callback: Arc::new(#fn_name_ident),
            }
        }
    };

    TokenStream::from(expanded)
}
