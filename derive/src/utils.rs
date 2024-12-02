use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::Ident;

pub fn get_wrapper_source() -> proc_macro2::TokenStream {
    match crate_name("async-graphql-semantic-nullability").unwrap() {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
    }
}
