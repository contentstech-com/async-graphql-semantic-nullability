use std::borrow::Cow;

use async_graphql::{
    context::ContextSelectionSet,
    parser::types::Field,
    registry::{Registry, SemanticNullability},
    OutputType, Positioned, ServerResult, Value,
};

pub use async_graphql_semantic_nullability_derive::*;

/// A wrapper type that can be used to mark a field as semantically non-nullable.
#[repr(transparent)]
#[derive(Debug)]
pub struct SemanticNonNull<T: ?Sized>(pub T);

impl<T: OutputType> OutputType for SemanticNonNull<T> {
    fn type_name() -> Cow<'static, str> {
        T::type_name()
    }

    fn qualified_type_name() -> String {
        T::type_name().to_string()
    }

    fn semantic_nullability() -> SemanticNullability {
        match T::semantic_nullability() {
            SemanticNullability::None => SemanticNullability::OutNonNull,
            SemanticNullability::OutNonNull => SemanticNullability::OutNonNull,
            SemanticNullability::InNonNull => SemanticNullability::BothNonNull,
            SemanticNullability::BothNonNull => SemanticNullability::BothNonNull,
        }
    }

    fn create_type_info(registry: &mut Registry) -> String {
        T::create_type_info(registry);
        T::type_name().to_string()
    }

    async fn resolve(
        &self,
        ctx: &ContextSelectionSet<'_>,
        field: &Positioned<Field>,
    ) -> ServerResult<Value> {
        self.0.resolve(ctx, field).await
    }
}

/// A wrapper type that can be used to mark a field as strictly non-nullable.
///
/// Wrapping a nullable type with this will result in a runtime error.
#[repr(transparent)]
#[derive(Debug)]
pub struct StrictNonNull<T: ?Sized>(pub T);

impl<T: OutputType> OutputType for StrictNonNull<T> {
    fn type_name() -> Cow<'static, str> {
        T::type_name()
    }

    fn qualified_type_name() -> String {
        format!("{}!", T::type_name())
    }

    fn semantic_nullability() -> SemanticNullability {
        SemanticNullability::None
    }

    fn create_type_info(registry: &mut Registry) -> String {
        T::create_type_info(registry);
        Self::qualified_type_name()
    }

    async fn resolve(
        &self,
        ctx: &ContextSelectionSet<'_>,
        field: &Positioned<Field>,
    ) -> ServerResult<Value> {
        match self.0.resolve(ctx, field).await {
            Ok(Value::Null) => Err(async_graphql::ServerError::new(
                "Expected to return non-null value, but got null",
                Some(field.pos),
            )),
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}

macro_rules! impl_traits {
    ($ident:ident) => {
        impl<T: std::fmt::Display + ?Sized> std::fmt::Display for $ident<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<T: std::hash::Hash + ?Sized> std::hash::Hash for $ident<T> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state)
            }
        }

        impl<T: PartialEq + ?Sized> PartialEq for $ident<T> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl<T: Eq + ?Sized> Eq for $ident<T> {}
    };
}

impl_traits!(SemanticNonNull);
impl_traits!(StrictNonNull);
