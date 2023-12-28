/// Defines a common struct trait macro.
/// This macro generates a struct with the specified name and fields.
/// The generated struct derives the `Debug` and `Deserialize` traits.
#[macro_export]
macro_rules! common_struct_trait {
    ($name:ident, $($field:ident: $type:ty),*) => {
        #[derive(Debug, Deserialize,Clone)]
        pub struct $name {
            $(pub $field: $type),*
        }
    };
}

/// Defines an engine struct trait macro.
/// This macro generates a struct with the specified name and fields.
/// The generated struct contains a vector of rules of type `$rule_type`.
/// The struct has a constructor method `new` that takes a vector of rules as input.
/// The struct has a `process` method that returns a vector of type `$return_type`.
#[macro_export]
macro_rules! engine_struct_trait {
    ($name:ident, $rule_type:ty, $return_type:ty) => {
        pub struct $name {
            rules: Vec<$rule_type>,
        }

        impl $name {
            /// Creates a new instance of the struct with the given rules.
            pub fn new(rules: Vec<$rule_type>) -> Self {
                Self { rules }
            }

            /// Processes the rules and returns the result as a vector.
            /// Returns a `Result` with the vector of `$return_type` or an error.
            pub fn process(&self) -> Result<Vec<$return_type>, Box<dyn std::error::Error>> {
                // Implement your logic here
                Ok(Vec::new())
            }
        }
    };
}
