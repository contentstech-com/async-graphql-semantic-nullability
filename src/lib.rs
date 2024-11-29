use std::borrow::Cow;

use async_graphql::{
    context::ContextSelectionSet,
    parser::types::Field,
    registry::{Registry, SemanticNullability},
    OutputType, Positioned, ServerResult, Value,
};

pub use async_graphql_semantic_nullability_derive::*;

#[repr(transparent)]
#[derive(Debug)]
pub struct SemanticNonNull<T>(pub T);

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
