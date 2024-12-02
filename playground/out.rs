#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::time::Duration;
use async_graphql::{Object, SimpleObject, Subscription};
use async_graphql_semantic_nullability::SemanticNonNull;
use tokio_stream::{Stream, StreamExt as _};
struct MySimpleObject {
    foo: String,
}
#[allow(clippy::all, clippy::pedantic)]
impl MySimpleObject {
    #[inline]
    #[allow(missing_docs)]
    async fn foo(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<&String> {
        ::std::result::Result::Ok(&self.foo)
    }
}
#[allow(clippy::all, clippy::pedantic)]
impl async_graphql::resolver_utils::ContainerType for MySimpleObject {
    async fn resolve_field(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
        if ctx.item.node.name.node == "foo" {
            let f = async move {
                self.foo(ctx).await.map_err(|err| err.into_server_error(ctx.item.pos))
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        ::std::result::Result::Ok(::std::option::Option::None)
    }
}
#[allow(clippy::all, clippy::pedantic)]
impl async_graphql::OutputType for MySimpleObject {
    fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
        ::std::borrow::Cow::Borrowed("MySimpleObject")
    }
    fn create_type_info(
        registry: &mut async_graphql::registry::Registry,
    ) -> ::std::string::String {
        registry
            .create_output_type::<
                Self,
                _,
            >(
                async_graphql::registry::MetaTypeId::Object,
                |registry| async_graphql::registry::MetaType::Object {
                    name: ::std::borrow::Cow::into_owned(
                        ::std::borrow::Cow::Borrowed("MySimpleObject"),
                    ),
                    description: ::std::option::Option::None,
                    fields: {
                        let mut fields = async_graphql::indexmap::IndexMap::new();
                        fields
                            .insert(
                                ::std::borrow::ToOwned::to_owned("foo"),
                                async_graphql::registry::MetaField {
                                    name: ::std::borrow::ToOwned::to_owned("foo"),
                                    description: ::std::option::Option::None,
                                    args: ::std::default::Default::default(),
                                    ty: <String as async_graphql::OutputType>::create_type_info(
                                        registry,
                                    ),
                                    deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                    cache_control: async_graphql::CacheControl {
                                        public: true,
                                        max_age: 0i32,
                                    },
                                    external: false,
                                    provides: ::std::option::Option::None,
                                    requires: ::std::option::Option::None,
                                    shareable: false,
                                    inaccessible: false,
                                    tags: ::alloc::vec::Vec::new(),
                                    override_from: ::std::option::Option::None,
                                    visible: ::std::option::Option::None,
                                    compute_complexity: ::std::option::Option::None,
                                    directive_invocations: ::alloc::vec::Vec::new(),
                                    semantic_nullability: async_graphql::registry::SemanticNullability::None,
                                },
                            );
                        fields
                    },
                    cache_control: async_graphql::CacheControl {
                        public: true,
                        max_age: 0i32,
                    },
                    extends: false,
                    shareable: false,
                    resolvable: true,
                    inaccessible: false,
                    interface_object: false,
                    tags: ::alloc::vec::Vec::new(),
                    keys: ::std::option::Option::None,
                    visible: ::std::option::Option::None,
                    is_subscription: false,
                    rust_typename: ::std::option::Option::Some(
                        ::std::any::type_name::<Self>(),
                    ),
                    directive_invocations: ::alloc::vec::Vec::new(),
                },
            )
    }
    async fn resolve(
        &self,
        ctx: &async_graphql::ContextSelectionSet<'_>,
        _field: &async_graphql::Positioned<async_graphql::parser::types::Field>,
    ) -> async_graphql::ServerResult<async_graphql::Value> {
        async_graphql::resolver_utils::resolve_container(ctx, self).await
    }
}
impl async_graphql::ObjectType for MySimpleObject {}
struct MyObject;
impl MyObject {
    async fn owned(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<
        ::async_graphql_semantic_nullability::SemanticNonNull<MySimpleObject>,
    > {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: ::async_graphql_semantic_nullability::SemanticNonNull<
                        MySimpleObject,
                    > = {
                        let result: MySimpleObject = {
                            MySimpleObject {
                                foo: "bar".to_string(),
                            }
                        };
                        unsafe {
                            ::std::mem::transmute::<
                                _,
                                ::async_graphql_semantic_nullability::SemanticNonNull<
                                    MySimpleObject,
                                >,
                            >(result)
                        }
                    };
                    value
                }
                    .await,
            )
        }
    }
    async fn borrowed(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<
        ::async_graphql_semantic_nullability::SemanticNonNull<&MySimpleObject>,
    > {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: ::async_graphql_semantic_nullability::SemanticNonNull<
                        &MySimpleObject,
                    > = {
                        let result: &MySimpleObject = {
                            &MySimpleObject {
                                foo: "bar".to_string(),
                            }
                        };
                        unsafe {
                            ::std::mem::transmute::<
                                _,
                                ::async_graphql_semantic_nullability::SemanticNonNull<
                                    &MySimpleObject,
                                >,
                            >(result)
                        }
                    };
                    value
                }
                    .await,
            )
        }
    }
    async fn array(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<
        ::async_graphql_semantic_nullability::SemanticNonNull<
            [::async_graphql_semantic_nullability::SemanticNonNull<MySimpleObject>; 3],
        >,
    > {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: ::async_graphql_semantic_nullability::SemanticNonNull<
                        [::async_graphql_semantic_nullability::SemanticNonNull<
                            MySimpleObject,
                        >; 3],
                    > = {
                        let result: [MySimpleObject; 3] = {
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
                        };
                        unsafe {
                            ::std::mem::transmute::<
                                _,
                                ::async_graphql_semantic_nullability::SemanticNonNull<
                                    [::async_graphql_semantic_nullability::SemanticNonNull<
                                        MySimpleObject,
                                    >; 3],
                                >,
                            >(result)
                        }
                    };
                    value
                }
                    .await,
            )
        }
    }
    async fn slice(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<
        ::async_graphql_semantic_nullability::SemanticNonNull<
            &[::async_graphql_semantic_nullability::SemanticNonNull<MySimpleObject>],
        >,
    > {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: ::async_graphql_semantic_nullability::SemanticNonNull<
                        &[::async_graphql_semantic_nullability::SemanticNonNull<
                            MySimpleObject,
                        >],
                    > = {
                        let result: &[MySimpleObject] = {
                            &[
                                MySimpleObject {
                                    foo: "bar".to_string(),
                                },
                                MySimpleObject {
                                    foo: "baz".to_string(),
                                },
                            ]
                        };
                        unsafe {
                            ::std::mem::transmute::<
                                _,
                                ::async_graphql_semantic_nullability::SemanticNonNull<
                                    &[::async_graphql_semantic_nullability::SemanticNonNull<
                                        MySimpleObject,
                                    >],
                                >,
                            >(result)
                        }
                    };
                    value
                }
                    .await,
            )
        }
    }
    async fn vec(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<
        ::async_graphql_semantic_nullability::SemanticNonNull<
            Vec<::async_graphql_semantic_nullability::SemanticNonNull<MySimpleObject>>,
        >,
    > {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: ::async_graphql_semantic_nullability::SemanticNonNull<
                        Vec<
                            ::async_graphql_semantic_nullability::SemanticNonNull<
                                MySimpleObject,
                            >,
                        >,
                    > = {
                        let result: Vec<MySimpleObject> = {
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    MySimpleObject {
                                        foo: "bar".to_string(),
                                    },
                                    MySimpleObject {
                                        foo: "baz".to_string(),
                                    },
                                ]),
                            )
                        };
                        unsafe {
                            ::std::mem::transmute::<
                                _,
                                ::async_graphql_semantic_nullability::SemanticNonNull<
                                    Vec<
                                        ::async_graphql_semantic_nullability::SemanticNonNull<
                                            MySimpleObject,
                                        >,
                                    >,
                                >,
                            >(result)
                        }
                    };
                    value
                }
                    .await,
            )
        }
    }
    async fn strict(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<
        Option<::async_graphql_semantic_nullability::StrictNonNull<MySimpleObject>>,
    > {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: Option<
                        ::async_graphql_semantic_nullability::StrictNonNull<
                            MySimpleObject,
                        >,
                    > = {
                        let result: Option<MySimpleObject> = {
                            Some(MySimpleObject {
                                foo: "bar".to_string(),
                            })
                        };
                        unsafe {
                            ::std::mem::transmute::<
                                _,
                                Option<
                                    ::async_graphql_semantic_nullability::StrictNonNull<
                                        MySimpleObject,
                                    >,
                                >,
                            >(result)
                        }
                    };
                    value
                }
                    .await,
            )
        }
    }
    async fn option(
        &self,
        _: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<
        Option<
            ::async_graphql_semantic_nullability::SemanticNonNull<Vec<MySimpleObject>>,
        >,
    > {
        {
            ::std::result::Result::Ok(
                async move {
                    let value: Option<
                        ::async_graphql_semantic_nullability::SemanticNonNull<
                            Vec<MySimpleObject>,
                        >,
                    > = {
                        let result: Option<Vec<MySimpleObject>> = {
                            Some(::alloc::vec::Vec::new())
                        };
                        unsafe {
                            ::std::mem::transmute::<
                                _,
                                Option<
                                    ::async_graphql_semantic_nullability::SemanticNonNull<
                                        Vec<MySimpleObject>,
                                    >,
                                >,
                            >(result)
                        }
                    };
                    value
                }
                    .await,
            )
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    enum __FieldIdent {
        owned,
        borrowed,
        array,
        slice,
        vec,
        strict,
        option,
    }
    impl __FieldIdent {
        fn from_name(
            __name: &async_graphql::Name,
        ) -> ::std::option::Option<__FieldIdent> {
            match __name.as_str() {
                "owned" => ::std::option::Option::Some(__FieldIdent::owned),
                "borrowed" => ::std::option::Option::Some(__FieldIdent::borrowed),
                "array" => ::std::option::Option::Some(__FieldIdent::array),
                "slice" => ::std::option::Option::Some(__FieldIdent::slice),
                "vec" => ::std::option::Option::Some(__FieldIdent::vec),
                "strict" => ::std::option::Option::Some(__FieldIdent::strict),
                "option" => ::std::option::Option::Some(__FieldIdent::option),
                _ => ::std::option::Option::None,
            }
        }
    }
    impl MyObject {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __owned_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.owned(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __borrowed_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.borrowed(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __array_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.array(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __slice_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.slice(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __vec_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.vec(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __strict_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.strict(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        async fn __option_resolver(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let f = async {
                let res = self.option(ctx).await;
                res.map_err(|err| {
                    ::std::convert::Into::<async_graphql::Error>::into(err)
                        .into_server_error(ctx.item.pos)
                })
            };
            let obj = f.await.map_err(|err| ctx.set_error_path(err))?;
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return async_graphql::OutputType::resolve(&obj, &ctx_obj, ctx.item)
                .await
                .map(::std::option::Option::Some);
        }
    }
    #[allow(clippy::all, clippy::pedantic, clippy::suspicious_else_formatting)]
    #[allow(unused_braces, unused_variables, unused_parens, unused_mut)]
    impl async_graphql::resolver_utils::ContainerType for MyObject {
        async fn resolve_field(
            &self,
            ctx: &async_graphql::Context<'_>,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let __field = __FieldIdent::from_name(&ctx.item.node.name.node);
            match __field {
                ::std::option::Option::Some(__FieldIdent::owned) => {
                    return self.__owned_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::borrowed) => {
                    return self.__borrowed_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::array) => {
                    return self.__array_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::slice) => {
                    return self.__slice_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::vec) => {
                    return self.__vec_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::strict) => {
                    return self.__strict_resolver(&ctx).await;
                }
                ::std::option::Option::Some(__FieldIdent::option) => {
                    return self.__option_resolver(&ctx).await;
                }
                None => {}
            }
            ::std::result::Result::Ok(::std::option::Option::None)
        }
        async fn find_entity(
            &self,
            ctx: &async_graphql::Context<'_>,
            params: &async_graphql::Value,
        ) -> async_graphql::ServerResult<::std::option::Option<async_graphql::Value>> {
            let params = match params {
                async_graphql::Value::Object(params) => params,
                _ => return ::std::result::Result::Ok(::std::option::Option::None),
            };
            let typename = if let ::std::option::Option::Some(
                async_graphql::Value::String(typename),
            ) = params.get("__typename")
            {
                typename
            } else {
                return ::std::result::Result::Err(
                    async_graphql::ServerError::new(
                        r#""__typename" must be an existing string."#,
                        ::std::option::Option::Some(ctx.item.pos),
                    ),
                );
            };
            ::std::result::Result::Ok(::std::option::Option::None)
        }
    }
    #[allow(clippy::all, clippy::pedantic)]
    impl async_graphql::OutputType for MyObject {
        fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
            ::std::borrow::Cow::Borrowed("MyObject")
        }
        fn create_type_info(
            registry: &mut async_graphql::registry::Registry,
        ) -> ::std::string::String {
            let ty = registry
                .create_output_type::<
                    Self,
                    _,
                >(
                    async_graphql::registry::MetaTypeId::Object,
                    |registry| async_graphql::registry::MetaType::Object {
                        name: ::std::borrow::Cow::into_owned(
                            ::std::borrow::Cow::Borrowed("MyObject"),
                        ),
                        description: ::std::option::Option::None,
                        fields: {
                            let mut fields = async_graphql::indexmap::IndexMap::new();
                            fields
                                .insert(
                                    ::std::borrow::ToOwned::to_owned("owned"),
                                    async_graphql::registry::MetaField {
                                        name: ::std::borrow::ToOwned::to_owned("owned"),
                                        description: ::std::option::Option::None,
                                        args: {
                                            let mut args = async_graphql::indexmap::IndexMap::new();
                                            args
                                        },
                                        ty: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            MySimpleObject,
                                        > as async_graphql::OutputType>::create_type_info(registry),
                                        deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                        cache_control: async_graphql::CacheControl {
                                            public: true,
                                            max_age: 0i32,
                                        },
                                        external: false,
                                        provides: ::std::option::Option::None,
                                        requires: ::std::option::Option::None,
                                        shareable: false,
                                        inaccessible: false,
                                        tags: ::alloc::vec::Vec::new(),
                                        override_from: ::std::option::Option::None,
                                        visible: ::std::option::Option::None,
                                        compute_complexity: ::std::option::Option::None,
                                        directive_invocations: ::alloc::vec::Vec::new(),
                                        semantic_nullability: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            MySimpleObject,
                                        > as async_graphql::OutputType>::semantic_nullability(),
                                    },
                                );
                            fields
                                .insert(
                                    ::std::borrow::ToOwned::to_owned("borrowed"),
                                    async_graphql::registry::MetaField {
                                        name: ::std::borrow::ToOwned::to_owned("borrowed"),
                                        description: ::std::option::Option::None,
                                        args: {
                                            let mut args = async_graphql::indexmap::IndexMap::new();
                                            args
                                        },
                                        ty: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            &MySimpleObject,
                                        > as async_graphql::OutputType>::create_type_info(registry),
                                        deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                        cache_control: async_graphql::CacheControl {
                                            public: true,
                                            max_age: 0i32,
                                        },
                                        external: false,
                                        provides: ::std::option::Option::None,
                                        requires: ::std::option::Option::None,
                                        shareable: false,
                                        inaccessible: false,
                                        tags: ::alloc::vec::Vec::new(),
                                        override_from: ::std::option::Option::None,
                                        visible: ::std::option::Option::None,
                                        compute_complexity: ::std::option::Option::None,
                                        directive_invocations: ::alloc::vec::Vec::new(),
                                        semantic_nullability: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            &MySimpleObject,
                                        > as async_graphql::OutputType>::semantic_nullability(),
                                    },
                                );
                            fields
                                .insert(
                                    ::std::borrow::ToOwned::to_owned("array"),
                                    async_graphql::registry::MetaField {
                                        name: ::std::borrow::ToOwned::to_owned("array"),
                                        description: ::std::option::Option::None,
                                        args: {
                                            let mut args = async_graphql::indexmap::IndexMap::new();
                                            args
                                        },
                                        ty: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            [::async_graphql_semantic_nullability::SemanticNonNull<
                                                MySimpleObject,
                                            >; 3],
                                        > as async_graphql::OutputType>::create_type_info(registry),
                                        deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                        cache_control: async_graphql::CacheControl {
                                            public: true,
                                            max_age: 0i32,
                                        },
                                        external: false,
                                        provides: ::std::option::Option::None,
                                        requires: ::std::option::Option::None,
                                        shareable: false,
                                        inaccessible: false,
                                        tags: ::alloc::vec::Vec::new(),
                                        override_from: ::std::option::Option::None,
                                        visible: ::std::option::Option::None,
                                        compute_complexity: ::std::option::Option::None,
                                        directive_invocations: ::alloc::vec::Vec::new(),
                                        semantic_nullability: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            [::async_graphql_semantic_nullability::SemanticNonNull<
                                                MySimpleObject,
                                            >; 3],
                                        > as async_graphql::OutputType>::semantic_nullability(),
                                    },
                                );
                            fields
                                .insert(
                                    ::std::borrow::ToOwned::to_owned("slice"),
                                    async_graphql::registry::MetaField {
                                        name: ::std::borrow::ToOwned::to_owned("slice"),
                                        description: ::std::option::Option::None,
                                        args: {
                                            let mut args = async_graphql::indexmap::IndexMap::new();
                                            args
                                        },
                                        ty: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            &[::async_graphql_semantic_nullability::SemanticNonNull<
                                                MySimpleObject,
                                            >],
                                        > as async_graphql::OutputType>::create_type_info(registry),
                                        deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                        cache_control: async_graphql::CacheControl {
                                            public: true,
                                            max_age: 0i32,
                                        },
                                        external: false,
                                        provides: ::std::option::Option::None,
                                        requires: ::std::option::Option::None,
                                        shareable: false,
                                        inaccessible: false,
                                        tags: ::alloc::vec::Vec::new(),
                                        override_from: ::std::option::Option::None,
                                        visible: ::std::option::Option::None,
                                        compute_complexity: ::std::option::Option::None,
                                        directive_invocations: ::alloc::vec::Vec::new(),
                                        semantic_nullability: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            &[::async_graphql_semantic_nullability::SemanticNonNull<
                                                MySimpleObject,
                                            >],
                                        > as async_graphql::OutputType>::semantic_nullability(),
                                    },
                                );
                            fields
                                .insert(
                                    ::std::borrow::ToOwned::to_owned("vec"),
                                    async_graphql::registry::MetaField {
                                        name: ::std::borrow::ToOwned::to_owned("vec"),
                                        description: ::std::option::Option::None,
                                        args: {
                                            let mut args = async_graphql::indexmap::IndexMap::new();
                                            args
                                        },
                                        ty: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            Vec<
                                                ::async_graphql_semantic_nullability::SemanticNonNull<
                                                    MySimpleObject,
                                                >,
                                            >,
                                        > as async_graphql::OutputType>::create_type_info(registry),
                                        deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                        cache_control: async_graphql::CacheControl {
                                            public: true,
                                            max_age: 0i32,
                                        },
                                        external: false,
                                        provides: ::std::option::Option::None,
                                        requires: ::std::option::Option::None,
                                        shareable: false,
                                        inaccessible: false,
                                        tags: ::alloc::vec::Vec::new(),
                                        override_from: ::std::option::Option::None,
                                        visible: ::std::option::Option::None,
                                        compute_complexity: ::std::option::Option::None,
                                        directive_invocations: ::alloc::vec::Vec::new(),
                                        semantic_nullability: <::async_graphql_semantic_nullability::SemanticNonNull<
                                            Vec<
                                                ::async_graphql_semantic_nullability::SemanticNonNull<
                                                    MySimpleObject,
                                                >,
                                            >,
                                        > as async_graphql::OutputType>::semantic_nullability(),
                                    },
                                );
                            fields
                                .insert(
                                    ::std::borrow::ToOwned::to_owned("strict"),
                                    async_graphql::registry::MetaField {
                                        name: ::std::borrow::ToOwned::to_owned("strict"),
                                        description: ::std::option::Option::None,
                                        args: {
                                            let mut args = async_graphql::indexmap::IndexMap::new();
                                            args
                                        },
                                        ty: <Option<
                                            ::async_graphql_semantic_nullability::StrictNonNull<
                                                MySimpleObject,
                                            >,
                                        > as async_graphql::OutputType>::create_type_info(registry),
                                        deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                        cache_control: async_graphql::CacheControl {
                                            public: true,
                                            max_age: 0i32,
                                        },
                                        external: false,
                                        provides: ::std::option::Option::None,
                                        requires: ::std::option::Option::None,
                                        shareable: false,
                                        inaccessible: false,
                                        tags: ::alloc::vec::Vec::new(),
                                        override_from: ::std::option::Option::None,
                                        visible: ::std::option::Option::None,
                                        compute_complexity: ::std::option::Option::None,
                                        directive_invocations: ::alloc::vec::Vec::new(),
                                        semantic_nullability: <Option<
                                            ::async_graphql_semantic_nullability::StrictNonNull<
                                                MySimpleObject,
                                            >,
                                        > as async_graphql::OutputType>::semantic_nullability(),
                                    },
                                );
                            fields
                                .insert(
                                    ::std::borrow::ToOwned::to_owned("option"),
                                    async_graphql::registry::MetaField {
                                        name: ::std::borrow::ToOwned::to_owned("option"),
                                        description: ::std::option::Option::None,
                                        args: {
                                            let mut args = async_graphql::indexmap::IndexMap::new();
                                            args
                                        },
                                        ty: <Option<
                                            ::async_graphql_semantic_nullability::SemanticNonNull<
                                                Vec<MySimpleObject>,
                                            >,
                                        > as async_graphql::OutputType>::create_type_info(registry),
                                        deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                        cache_control: async_graphql::CacheControl {
                                            public: true,
                                            max_age: 0i32,
                                        },
                                        external: false,
                                        provides: ::std::option::Option::None,
                                        requires: ::std::option::Option::None,
                                        shareable: false,
                                        inaccessible: false,
                                        tags: ::alloc::vec::Vec::new(),
                                        override_from: ::std::option::Option::None,
                                        visible: ::std::option::Option::None,
                                        compute_complexity: ::std::option::Option::None,
                                        directive_invocations: ::alloc::vec::Vec::new(),
                                        semantic_nullability: <Option<
                                            ::async_graphql_semantic_nullability::SemanticNonNull<
                                                Vec<MySimpleObject>,
                                            >,
                                        > as async_graphql::OutputType>::semantic_nullability(),
                                    },
                                );
                            fields
                        },
                        cache_control: async_graphql::CacheControl {
                            public: true,
                            max_age: 0i32,
                        },
                        extends: false,
                        shareable: false,
                        resolvable: true,
                        inaccessible: false,
                        interface_object: false,
                        tags: ::alloc::vec::Vec::new(),
                        keys: ::std::option::Option::None,
                        visible: ::std::option::Option::None,
                        is_subscription: false,
                        rust_typename: ::std::option::Option::Some(
                            ::std::any::type_name::<Self>(),
                        ),
                        directive_invocations: ::alloc::vec::Vec::new(),
                    },
                );
            ty
        }
        async fn resolve(
            &self,
            ctx: &async_graphql::ContextSelectionSet<'_>,
            _field: &async_graphql::Positioned<async_graphql::parser::types::Field>,
        ) -> async_graphql::ServerResult<async_graphql::Value> {
            async_graphql::resolver_utils::resolve_container(ctx, self).await
        }
    }
    impl async_graphql::ObjectType for MyObject {}
};
struct Subscription;
impl Subscription {
    async fn integers(
        &self,
        _: &async_graphql::Context<'_>,
        step: i32,
    ) -> async_graphql::Result<
        impl Stream<Item = ::async_graphql_semantic_nullability::SemanticNonNull<i32>>,
    > {
        {
            let value = (move || {
                async move {
                    let result = {
                        let mut value = 0;
                        tokio_stream::wrappers::IntervalStream::new(
                                tokio::time::interval(Duration::from_secs(1)),
                            )
                            .map(move |_| {
                                value += step;
                                value
                            })
                    };
                    ::tokio_stream::StreamExt::map(
                        result,
                        |v| unsafe {
                            ::std::mem::transmute::<
                                i32,
                                ::async_graphql_semantic_nullability::SemanticNonNull<i32>,
                            >(v)
                        },
                    )
                }
            })()
                .await;
            ::std::result::Result::Ok(value)
        }
    }
}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_braces, unused_variables)]
impl async_graphql::SubscriptionType for Subscription {
    fn type_name() -> ::std::borrow::Cow<'static, ::std::primitive::str> {
        ::std::borrow::Cow::Borrowed("Subscription")
    }
    #[allow(bare_trait_objects)]
    fn create_type_info(
        registry: &mut async_graphql::registry::Registry,
    ) -> ::std::string::String {
        registry
            .create_subscription_type::<
                Self,
                _,
            >(|registry| async_graphql::registry::MetaType::Object {
                name: ::std::borrow::Cow::into_owned(
                    ::std::borrow::Cow::Borrowed("Subscription"),
                ),
                description: ::std::option::Option::None,
                fields: {
                    let mut fields = async_graphql::indexmap::IndexMap::new();
                    fields
                        .insert(
                            ::std::borrow::ToOwned::to_owned("integers"),
                            async_graphql::registry::MetaField {
                                name: ::std::borrow::ToOwned::to_owned("integers"),
                                description: ::std::option::Option::None,
                                args: {
                                    let mut args = async_graphql::indexmap::IndexMap::new();
                                    args.insert(
                                        ::std::borrow::ToOwned::to_owned("step"),
                                        async_graphql::registry::MetaInputValue {
                                            name: ::std::string::ToString::to_string("step"),
                                            description: ::std::option::Option::None,
                                            ty: <i32 as async_graphql::InputType>::create_type_info(
                                                registry,
                                            ),
                                            default_value: ::std::option::Option::Some(
                                                ::std::string::ToString::to_string(
                                                    &<i32 as async_graphql::InputType>::to_value(
                                                        &{
                                                            ::std::convert::TryInto::try_into(1i32).unwrap_or_default()
                                                        },
                                                    ),
                                                ),
                                            ),
                                            visible: ::std::option::Option::None,
                                            inaccessible: false,
                                            tags: ::std::default::Default::default(),
                                            is_secret: false,
                                            directive_invocations: ::alloc::vec::Vec::new(),
                                        },
                                    );
                                    args
                                },
                                ty: <<dyn Stream<
                                    Item = ::async_graphql_semantic_nullability::SemanticNonNull<
                                        i32,
                                    >,
                                > as async_graphql::futures_util::stream::Stream>::Item as async_graphql::OutputType>::create_type_info(
                                    registry,
                                ),
                                deprecation: async_graphql::registry::Deprecation::NoDeprecated,
                                cache_control: ::std::default::Default::default(),
                                external: false,
                                requires: ::std::option::Option::None,
                                provides: ::std::option::Option::None,
                                shareable: false,
                                override_from: ::std::option::Option::None,
                                visible: ::std::option::Option::None,
                                inaccessible: false,
                                tags: ::std::default::Default::default(),
                                compute_complexity: ::std::option::Option::None,
                                directive_invocations: ::alloc::vec::Vec::new(),
                                semantic_nullability: <<dyn Stream<
                                    Item = ::async_graphql_semantic_nullability::SemanticNonNull<
                                        i32,
                                    >,
                                > as async_graphql::futures_util::stream::Stream>::Item as async_graphql::OutputType>::semantic_nullability(),
                            },
                        );
                    fields
                },
                cache_control: ::std::default::Default::default(),
                extends: false,
                keys: ::std::option::Option::None,
                visible: ::std::option::Option::None,
                shareable: false,
                resolvable: true,
                inaccessible: false,
                interface_object: false,
                tags: ::std::default::Default::default(),
                is_subscription: true,
                rust_typename: ::std::option::Option::Some(
                    ::std::any::type_name::<Self>(),
                ),
                directive_invocations: ::alloc::vec::Vec::new(),
            })
    }
    fn create_field_stream<'__life>(
        &'__life self,
        ctx: &'__life async_graphql::Context<'_>,
    ) -> ::std::option::Option<
        ::std::pin::Pin<
            ::std::boxed::Box<
                dyn async_graphql::futures_util::stream::Stream<
                    Item = async_graphql::Response,
                > + ::std::marker::Send + '__life,
            >,
        >,
    > {
        if ctx.item.node.name.node == "integers" {
            let stream = async_graphql::futures_util::stream::TryStreamExt::try_flatten(
                async_graphql::futures_util::stream::once(
                    (move || async move {
                        let field_name = ::std::clone::Clone::clone(
                            &ctx.item.node.response_key().node,
                        );
                        let field = ::std::sync::Arc::new(
                            ::std::clone::Clone::clone(&ctx.item),
                        );
                        let f = async {
                            #[allow(non_snake_case, unused_mut)]
                            let (__pos, mut step) = ctx
                                .param_value::<
                                    i32,
                                >(
                                    "step",
                                    ::std::option::Option::Some(|| -> i32 {
                                        {
                                            ::std::convert::TryInto::try_into(1i32).unwrap_or_default()
                                        }
                                    }),
                                )?;
                            #[allow(non_snake_case)]
                            let step = step;
                            self.integers(ctx, step)
                                .await
                                .map_err(|err| {
                                    ::std::convert::Into::<async_graphql::Error>::into(err)
                                        .into_server_error(ctx.item.pos)
                                        .with_path(
                                            <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([
                                                    async_graphql::PathSegment::Field(
                                                        ::std::borrow::ToOwned::to_owned(&*field_name),
                                                    ),
                                                ]),
                                            ),
                                        )
                                })
                        };
                        let stream = f.await.map_err(|err| ctx.set_error_path(err))?;
                        let pos = ctx.item.pos;
                        let schema_env = ::std::clone::Clone::clone(&ctx.schema_env);
                        let query_env = ::std::clone::Clone::clone(&ctx.query_env);
                        let stream = async_graphql::futures_util::stream::StreamExt::then(
                            stream,
                            {
                                let field_name = ::std::clone::Clone::clone(&field_name);
                                move |msg| {
                                    let schema_env = ::std::clone::Clone::clone(&schema_env);
                                    let query_env = ::std::clone::Clone::clone(&query_env);
                                    let field = ::std::clone::Clone::clone(&field);
                                    let field_name = ::std::clone::Clone::clone(&field_name);
                                    async move {
                                        let f = |
                                            execute_data: ::std::option::Option<async_graphql::Data>|
                                        {
                                            let schema_env = ::std::clone::Clone::clone(&schema_env);
                                            let query_env = ::std::clone::Clone::clone(&query_env);
                                            async move {
                                                let ctx_selection_set = query_env
                                                    .create_context(
                                                        &schema_env,
                                                        ::std::option::Option::Some(async_graphql::QueryPathNode {
                                                            parent: ::std::option::Option::None,
                                                            segment: async_graphql::QueryPathSegment::Name(&field_name),
                                                        }),
                                                        &field.node.selection_set,
                                                        execute_data.as_ref(),
                                                    );
                                                let parent_type = ::std::borrow::Cow::Borrowed(
                                                    "Subscription",
                                                );
                                                #[allow(bare_trait_objects)]
                                                let ri = async_graphql::extensions::ResolveInfo {
                                                    path_node: ctx_selection_set.path_node.as_ref().unwrap(),
                                                    parent_type: &parent_type,
                                                    return_type: &<<dyn Stream<
                                                        Item = ::async_graphql_semantic_nullability::SemanticNonNull<
                                                            i32,
                                                        >,
                                                    > as async_graphql::futures_util::stream::Stream>::Item as async_graphql::OutputType>::qualified_type_name(),
                                                    name: field.node.name.node.as_str(),
                                                    alias: field
                                                        .node
                                                        .alias
                                                        .as_ref()
                                                        .map(|alias| alias.node.as_str()),
                                                    is_for_introspection: false,
                                                    field: &field.node,
                                                };
                                                let resolve_fut = async {
                                                    async_graphql::OutputType::resolve(
                                                            &msg,
                                                            &ctx_selection_set,
                                                            &*field,
                                                        )
                                                        .await
                                                        .map(::std::option::Option::Some)
                                                };
                                                let mut resolve_fut = resolve_fut;
                                                #[allow(unused_mut)]
                                                let mut resolve_fut = unsafe {
                                                    ::pin_utils::core_reexport::pin::Pin::new_unchecked(
                                                        &mut resolve_fut,
                                                    )
                                                };
                                                let mut resp = query_env
                                                    .extensions
                                                    .resolve(ri, &mut resolve_fut)
                                                    .await
                                                    .map(|value| {
                                                        let mut map = async_graphql::indexmap::IndexMap::new();
                                                        map.insert(
                                                            ::std::clone::Clone::clone(&field_name),
                                                            value.unwrap_or_default(),
                                                        );
                                                        async_graphql::Response::new(
                                                            async_graphql::Value::Object(map),
                                                        )
                                                    })
                                                    .unwrap_or_else(|err| async_graphql::Response::from_errors(
                                                        <[_]>::into_vec(
                                                            #[rustc_box]
                                                            ::alloc::boxed::Box::new([err]),
                                                        ),
                                                    ));
                                                use ::std::iter::Extend;
                                                resp.errors
                                                    .extend(
                                                        ::std::mem::take(&mut *query_env.errors.lock().unwrap()),
                                                    );
                                                resp
                                            }
                                        };
                                        ::std::result::Result::Ok(
                                            query_env
                                                .extensions
                                                .execute(query_env.operation_name.as_deref(), f)
                                                .await,
                                        )
                                    }
                                }
                            },
                        );
                        async_graphql::ServerResult::Ok(stream)
                    })(),
                ),
            );
            let stream = async_graphql::futures_util::StreamExt::map(
                stream,
                |res| match res {
                    ::std::result::Result::Ok(resp) => resp,
                    ::std::result::Result::Err(err) => {
                        async_graphql::Response::from_errors(
                            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([err])),
                        )
                    }
                },
            );
            return ::std::option::Option::Some(::std::boxed::Box::pin(stream));
        }
        ::std::option::Option::None
    }
}
fn main() {}
