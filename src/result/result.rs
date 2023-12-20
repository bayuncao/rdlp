use std::collections::HashMap;

/// Represents a result with various fields.
pub struct Result {
    /*
       Fields from configuration file [[rules.item]]
    */
    id: u32,
    name: String,
    category: String,
    description: String,
    level: u32,

    /*
       Fields from configuration file [rules.mask]
    */
    symbol: String,
    length: u32,
    start_index: u32,

    /*
       Fields from the Result struct
    */
    text: String,
    mask_text: String,
}
