use darling::FromMeta;

#[derive(FromMeta, Default)]
#[darling(default)]
pub struct AttributeMeta {
    pub strict_non_null: bool,
}
