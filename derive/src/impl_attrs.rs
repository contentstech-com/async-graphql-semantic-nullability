use darling::FromMeta;
use proc_macro2::Span;
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
        let wrapper_ident = match attribute_meta.strict_non_null {
            true => &self.strict_non_null,
            false => &self.semantic_non_null,
        };

        let return_type = match &mut field.sig.output {
            ReturnType::Type(_, ty) => ty,
            ReturnType::Default => return,
        };
        let orig_return_type = return_type.clone();
        let Some((field_type, is_subscription)) = self.extract_field_type(return_type) else {
            return;
        };
        match Self::extract_inner_type(field_type) {
            Ok(inner) => {
                if let Some(inner) = inner {
                    self.wrap(inner, wrapper_ident);
                }
            }
            Err(err) => {
                self.errors.push(err);
                return;
            }
        };
        let (orig_field_type, new_field_type) = self.wrap(field_type, wrapper_ident);

        let body = &field.block;
        if is_subscription {
            field.block = parse_quote!({
                let result = #body;
                ::tokio_stream::StreamExt::map(result, |v| unsafe { ::std::mem::transmute::<#orig_field_type, #new_field_type>(v) })
            })
        } else {
            field.block = parse_quote!({
                let result: #orig_return_type = #body;
                unsafe { ::std::mem::transmute::<_, #return_type>(result) }
            })
        }
    }
}

impl TransformImpl {
    fn wrap(&self, ty: &mut Type, wrapper_ident: &Ident) -> (Type, Type) {
        let wrapper_source = &self.wrapper_source;
        let orig_ty = ty.clone();
        let new_ty: Type = parse_quote! { #wrapper_source::#wrapper_ident<#ty> };
        *ty = new_ty.clone();

        (orig_ty, new_ty)
    }

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

    fn extract_inner_type(ty: &mut Type) -> Result<Option<&mut Type>> {
        match ty {
            Type::Array(ty) => Ok(Some(&mut ty.elem)),
            Type::Slice(ty) => Ok(Some(&mut ty.elem)),
            Type::Paren(ty) => Self::extract_inner_type(&mut ty.elem),
            Type::Reference(ty) => Self::extract_inner_type(&mut ty.elem),
            Type::Path(ty) => match ty.path.segments.last_mut() {
                Some(path) => match path.ident.to_string().as_str() {
                    name @ ("BTreeSet" | "HashSet" | "LinkedList" | "Vec" | "VecDeque") => {
                        let span = path.span();
                        let PathArguments::AngleBracketed(args) = &mut path.arguments else {
                            return Err(Error::new(
                                span,
                                format!("`{}` should have angle bracketed generic arguments", name),
                            ));
                        };
                        match args.args.first_mut() {
                            Some(GenericArgument::Type(ty)) => Ok(Some(ty)),
                            _ => Err(Error::new(
                                span,
                                format!("`{}` should have one type argument", name),
                            )),
                        }
                    }
                    _ => Ok(None),
                },
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}
