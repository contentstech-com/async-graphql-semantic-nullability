use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemImpl, Meta};

use crate::impl_attrs::GraphQLAttrMacroType;

mod impl_attrs;
mod utils;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn SemanticNonNull(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(input as ItemImpl);

    let mut macro_type = None;
    for attr in &mut item_impl.attrs {
        let path = match &attr.meta {
            Meta::List(meta) => &meta.path,
            Meta::Path(meta) => meta,
            Meta::NameValue(_) => continue,
        };
        let current_type = match &path.segments.last().unwrap().ident {
            ident if ident == "Object" => GraphQLAttrMacroType::Object,
            ident if ident == "ComplexObject" => GraphQLAttrMacroType::ComplexObject,
            ident if ident == "Interface" => GraphQLAttrMacroType::Interface,
            ident if ident == "Subscription" => GraphQLAttrMacroType::Subscription,
            _ => continue,
        };
        if macro_type.is_some() {
            return syn::Error::new_spanned(attr, "Multiple async-graphql attribute macros found")
                .into_compile_error()
                .into();
        }
        *attr = match &mut attr.meta {
            Meta::Path(meta) => {
                let path = &meta.segments;
                parse_quote!(#[#path(semantic_non_null)])
            }
            Meta::List(meta) => {
                let path = &meta.path;
                let tokens = &meta.tokens;
                parse_quote!(#[#path(#tokens, semantic_non_null)])
            }
            _ => unreachable!(),
        };
        macro_type = Some(current_type);
    }

    let errors = match macro_type {
        Some(macro_type) => impl_attrs::transform_impl(&mut item_impl, macro_type).err(),
        None => Some(syn::Error::new_spanned(
            &item_impl,
            "Expected the impl block to have one of the supported async-graphql attribute macros",
        )),
    }
    .map(|err| err.to_compile_error());

    quote! {
        #item_impl
        #errors
    }
    .into()
}
