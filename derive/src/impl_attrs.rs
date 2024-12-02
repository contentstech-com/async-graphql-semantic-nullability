use darling::FromMeta;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_quote,
    spanned::Spanned,
    visit_mut::{visit_item_impl_mut, VisitMut},
    Error, GenericArgument, Ident, PathArguments, Result, ReturnType, Type, TypeParamBound,
};

use crate::{meta::AttributeMeta, utils::get_wrapper_source};

#[derive(PartialEq, Eq)]
pub enum GraphQLAttrMacroType {
    Object,
    ComplexObject,
    Interface,
    Subscription,
}

pub fn transform_impl(input: &mut syn::ItemImpl, macro_type: GraphQLAttrMacroType) -> Result<()> {
    let mut visitor = TransformImpl {
        wrapper_source: get_wrapper_source(),
        semantic_non_null: Ident::new("SemanticNonNull", Span::call_site()),
        strict_non_null: Ident::new("StrictNonNull", Span::call_site()),
        macro_type,
        errors: Vec::new(),
    };
    visit_item_impl_mut(&mut visitor, input);
    if visitor.errors.is_empty() {
        Ok(())
    } else {
        Err(visitor.errors.into_iter().next().unwrap())
    }
}

struct TransformImpl {
    wrapper_source: proc_macro2::TokenStream,
    semantic_non_null: Ident,
    strict_non_null: Ident,
    macro_type: GraphQLAttrMacroType,
    errors: Vec<Error>,
}

impl VisitMut for TransformImpl {
    fn visit_impl_item_fn_mut(&mut self, field: &mut syn::ImplItemFn) {
        let attribute_meta = if let Some((index, _)) =
            field.attrs.iter().enumerate().find(|(_, attr)| {
                attr.path()
                    .get_ident()
                    .map(|ident| ident == "semantic_nullability")
                    .unwrap_or(false)
            }) {
            let attr = field.attrs.remove(index);
            match AttributeMeta::from_meta(&attr.meta) {
                Ok(parsed) => parsed,
                Err(_) => {
                    self.errors.push(Error::new_spanned(
                        attr,
                        "Invalid attribute values for `semantic_nullability`",
                    ));
                    return;
                }
            }
        } else {
            AttributeMeta::default()
        };

        let return_type = match &mut field.sig.output {
            ReturnType::Type(_, ty) => ty,
            ReturnType::Default => return,
        };
        let orig_return_type = return_type.clone();
        let Some((field_type, is_subscription)) = self.extract_field_type(return_type) else {
            return;
        };
        let (orig_field_type, new_field_type) =
            match self.get_wrapped_type(field_type, attribute_meta.strict_non_null, true) {
                Ok(new_field_type) => {
                    let orig_field_type = field_type.clone();
                    *field_type = new_field_type.clone();
                    (orig_field_type, new_field_type)
                }
                Err(err) => {
                    self.errors.push(err);
                    return;
                }
            };

        let body = &field.block;
        if is_subscription {
            field.block = parse_quote!({
                let result = #body;
                #[allow(clippy::useless_transmute)]
                ::tokio_stream::StreamExt::map(result, |v| unsafe { ::std::mem::transmute::<#orig_field_type, #new_field_type>(v) })
            })
        } else {
            field.block = parse_quote!({
                let result: #orig_return_type = #body;
                #[allow(clippy::useless_transmute)]
                unsafe { ::std::mem::transmute::<_, #return_type>(result) }
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

    fn get_wrapped_type(&self, ty: &Type, is_strict: bool, needs_wrap: bool) -> Result<Type> {
        let new_ty = match ty {
            Type::Array(ty) => Type::Array(syn::TypeArray {
                elem: Box::new(self.get_wrapped_type(&ty.elem, is_strict, true)?),
                ..ty.clone()
            }),
            Type::Slice(ty) => Type::Slice(syn::TypeSlice {
                elem: Box::new(self.get_wrapped_type(&ty.elem, is_strict, true)?),
                ..ty.clone()
            }),
            Type::Paren(ty) => Type::Paren(syn::TypeParen {
                elem: Box::new(self.get_wrapped_type(&ty.elem, is_strict, false)?),
                ..ty.clone()
            }),
            Type::Reference(ty) => Type::Reference(syn::TypeReference {
                elem: Box::new(self.get_wrapped_type(&ty.elem, is_strict, false)?),
                ..ty.clone()
            }),
            Type::Path(ty) => {
                let mut ty = ty.clone();
                let Some(path) = ty.path.segments.last_mut() else {
                    return Err(Error::new(
                        ty.span(),
                        "Path should have at least one segment",
                    ));
                };
                let ident = path.ident.to_string();
                match &ident[..] {
                    name @ ("BTreeSet" | "HashSet" | "LinkedList" | "Vec" | "VecDeque"
                    | "Option" | "Result") => {
                        let PathArguments::AngleBracketed(args) = &mut path.arguments else {
                            return Err(Error::new(
                                path.span(),
                                format!("`{}` should have angle bracketed generic arguments", name),
                            ));
                        };
                        let Some(GenericArgument::Type(inner)) = args.args.first_mut() else {
                            return Err(Error::new(
                                path.span(),
                                format!("`{}` should have one type argument", name),
                            ));
                        };

                        let (should_wrap_inner, should_wrap_this) = match name {
                            "Option" | "Result" => (false, is_strict),
                            _ => (true, true),
                        };
                        *inner = self.get_wrapped_type(inner, is_strict, should_wrap_inner)?;
                        match should_wrap_this {
                            true => Type::Path(ty),
                            false => return Ok(Type::Path(ty)),
                        }
                    }
                    _ => Type::Path(ty),
                }
            }
            ty => ty.clone(),
        };

        match needs_wrap {
            true => {
                let wrapper_source = &self.wrapper_source;
                let wrapper_ident = match is_strict {
                    true => &self.strict_non_null,
                    false => &self.semantic_non_null,
                };
                let wrapper = quote!(#wrapper_source::#wrapper_ident);
                Ok(parse_quote!(#wrapper<#new_ty>))
            }
            false => Ok(new_ty),
        }
    }
}
