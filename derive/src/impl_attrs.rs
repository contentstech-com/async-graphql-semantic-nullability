use syn::{
    parse_quote,
    spanned::Spanned,
    visit_mut::{visit_item_impl_mut, VisitMut},
    Error, GenericArgument, PathArguments, Result, ReturnType, Type, TypeParamBound,
};

use crate::utils::get_semantic_non_null_wrapper;

#[derive(PartialEq, Eq)]
pub enum GraphQLAttrMacroType {
    Object,
    ComplexObject,
    Interface,
    Subscription,
}

pub fn transform_impl(input: &mut syn::ItemImpl, macro_type: GraphQLAttrMacroType) -> Result<()> {
    let mut visitor = TransformImpl {
        wrapper_name: get_semantic_non_null_wrapper(),
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
    wrapper_name: proc_macro2::TokenStream,
    macro_type: GraphQLAttrMacroType,
    errors: Vec<Error>,
}

impl VisitMut for TransformImpl {
    fn visit_impl_item_fn_mut(&mut self, field: &mut syn::ImplItemFn) {
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
                    self.wrap(inner);
                }
            }
            Err(err) => {
                self.errors.push(err);
                return;
            }
        };
        let (orig_field_type, new_field_type) = self.wrap(field_type);

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
    fn wrap(&self, ty: &mut Type) -> (Type, Type) {
        let wrapper_name = &self.wrapper_name;
        let orig_ty = ty.clone();
        let new_ty: Type = parse_quote! { #wrapper_name<#ty> };
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
