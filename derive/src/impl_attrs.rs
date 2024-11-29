use syn::{
    parse_quote,
    visit_mut::{visit_item_impl_mut, VisitMut},
    GenericArgument, PathArguments, ReturnType, Type, TypeParamBound,
};

use crate::utils::get_semantic_non_null_wrapper;

#[derive(PartialEq, Eq)]
pub enum GraphQLAttrMacroType {
    Object,
    ComplexObject,
    Interface,
    Subscription,
}

pub fn transform_impl(input: &mut syn::ItemImpl, macro_type: GraphQLAttrMacroType) {
    visit_item_impl_mut(
        &mut TransformImpl {
            wrapper_name: get_semantic_non_null_wrapper(),
            macro_type,
        },
        input,
    );
}

struct TransformImpl {
    wrapper_name: proc_macro2::TokenStream,
    macro_type: GraphQLAttrMacroType,
}

impl VisitMut for TransformImpl {
    fn visit_impl_item_fn_mut(&mut self, field: &mut syn::ImplItemFn) {
        let wrapper_name = &self.wrapper_name;
        let return_type = match &mut field.sig.output {
            ReturnType::Type(_, ty) => ty,
            ReturnType::Default => return,
        };
        let orig_return_type = return_type.clone();
        let (field_type, is_subscription) = match self.extract_field_type(return_type) {
            Some(ty) => ty,
            None => return,
        };
        let orig_field_type = field_type.clone();
        let new_field_type: Type = parse_quote! { #wrapper_name<#orig_field_type> };
        *field_type = new_field_type.clone();

        let body = &field.block;
        if is_subscription {
            field.block = parse_quote!({
                let result = #body;
                ::tokio_stream::StreamExt::map(result, |v| unsafe { ::std::mem::transmute::<#orig_field_type, #new_field_type>(v) })
            })
        } else {
            field.block = parse_quote!({
                let result = #body;
                unsafe { ::std::mem::transmute::<#orig_return_type, #return_type>(result) }
            })
        }
    }
}

impl TransformImpl {
    fn extract_field_type<'a>(&self, return_type: &'a mut Type) -> Option<(&'a mut Type, bool)> {
        match return_type {
            Type::Paren(ty) => self.extract_field_type(&mut ty.elem),
            ty @ (Type::Path(_) | Type::Array(_) | Type::Slice(_) | Type::Reference(_)) => {
                Some((ty, false))
            }
            Type::ImplTrait(ty) if self.macro_type == GraphQLAttrMacroType::Subscription => {
                ty.bounds.iter_mut().find_map(|bound| match bound {
                    TypeParamBound::Trait(bound) => {
                        bound.path.segments.last_mut().and_then(|segment| {
                            match &mut segment.arguments {
                                PathArguments::AngleBracketed(args)
                                    if segment.ident == "Stream" =>
                                {
                                    args.args.iter_mut().find_map(|arg| match arg {
                                        GenericArgument::AssocType(ty) if ty.ident == "Item" => {
                                            Some((&mut ty.ty, true))
                                        }
                                        _ => None,
                                    })
                                }
                                _ => None,
                            }
                        })
                    }
                    _ => None,
                })
            }
            _ => None,
        }
    }
}
