#[macro_export]
macro_rules! common_struct_trait {
    ($name:ident, $($field:ident: $type:ty),*) => {
        #[derive(Debug, Deserialize)]
        pub struct $name {
            $($field: $type),*
        }
    };
}
