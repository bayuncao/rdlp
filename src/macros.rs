#[macro_export]
macro_rules! common_struct_trait {
    ($name:ident, $($field:ident: $type:ty),*) => {
        #[derive(Debug, Deserialize)]
        pub struct $name {
            $($field: $type),*
        }
    };
}



#[macro_export]
macro_rules! engine_struct_trait {
    ($name:ident, $rule_type:ty, $return_type:ty) => {
        pub struct $name {
            rules: Vec<$rule_type>,
        }

        impl $name {
            pub fn new(rules: Vec<$rule_type>) -> Self {
                Self { rules }
            }

            pub fn process(&self) -> Result<Vec<$return_type>, Box<dyn std::error::Error>> {
                // Implement your logic here
                Ok(Vec::new())
            }
        }
    };
}