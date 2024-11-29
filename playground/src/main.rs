use std::time::Duration;

use async_graphql::{Object, SimpleObject, Subscription};
use async_graphql_semantic_nullability::SemanticNonNull;
use tokio_stream::{Stream, StreamExt as _};

#[derive(SimpleObject)]
struct MySimpleObject {
    foo: String,
}

struct MyObject;

#[SemanticNonNull]
#[Object]
impl MyObject {
    async fn my_simple_object(&self) -> MySimpleObject {
        MySimpleObject {
            foo: "bar".to_string(),
        }
    }
}

struct Subscription;

#[SemanticNonNull]
#[Subscription]
impl Subscription {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += step;
                value
            })
    }
}

fn main() {}
