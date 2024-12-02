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
    async fn owned(&self) -> MySimpleObject {
        MySimpleObject {
            foo: "bar".to_string(),
        }
    }

    async fn borrowed(&self) -> &MySimpleObject {
        &MySimpleObject {
            foo: "bar".to_string(),
        }
    }

    async fn array(&self) -> [MySimpleObject; 3] {
        [
            MySimpleObject {
                foo: "bar".to_string(),
            },
            MySimpleObject {
                foo: "bar".to_string(),
            },
            MySimpleObject {
                foo: "bar".to_string(),
            },
        ]
    }

    async fn slice(&self) -> &[MySimpleObject] {
        &[
            MySimpleObject {
                foo: "bar".to_string(),
            },
            MySimpleObject {
                foo: "baz".to_string(),
            },
        ]
    }

    async fn vec(&self) -> Vec<MySimpleObject> {
        vec![
            MySimpleObject {
                foo: "bar".to_string(),
            },
            MySimpleObject {
                foo: "baz".to_string(),
            },
        ]
    }

    #[semantic_nullability(strict_non_null)]
    async fn strict(&self) -> Option<MySimpleObject> {
        Some(MySimpleObject {
            foo: "bar".to_string(),
        })
    }

    async fn option(&self) -> Option<MySimpleObject> {
        None
    }
}

struct Subscription;

#[SemanticNonNull]
#[Subscription]
impl Subscription {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = Option<i32>> {
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                value += step;
                Some(value)
            })
    }
}

fn main() {}
