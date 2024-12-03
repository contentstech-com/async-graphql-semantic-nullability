# async-graphql-semantic-nullability

A crate for more convenient usage of semantic nullability in [async-graphql](https://github.com/async-graphql/async-graphql)

> [!WARNING]
> This crate assumes usage of `async-graphql` with [PR #1638](https://github.com/async-graphql/async-graphql/pull/1638), therefore will break otherwise.
>
> This crate also enables the `nullable-result` feature of `async-graphql` (only exists on top of [PR #1637](https://github.com/async-graphql/async-graphql/pull/1637)) so check carefully whether your existing schema gets any unintentional changes.

## Usage

```rust
use async_graphql::Object;
use async_graphql_semantic_nullability::SemanticNonNull;

struct MyObject;

// This will be transformed to...
#[SemanticNonNull]
#[Object]
impl MyObject {
    async fn field(&self) -> i32 {
        42
    }

    #[semantic_nullability(strict_non_null)]
    async fn strict_field(&self) -> i32 {
        42
    }
}

// ...something like this
#[Object]
impl MyObject {
    // This will now be `Int @semanticNonNull` instead of `Int!`
    async fn field(&self) -> SemanticNonNull<i32> {
        // This is not an actual output, but a similar one
        SemanticNonNull(42)
    }

    // This will still be `Int!`
    async fn strict_field(&self) -> StrictNonNull<i32> {
        // This is not an actual output, but a similar one
        StrictNonNull(42)
    }
}
```
