use std::collections::HashMap;

/// Represents a result object.
pub struct Result {
    id: i32,
    text: String,
    mask_text: String,
    result_type: String,
    key: String,
    byte_start: usize,
    byte_end: usize,
    info_type: String,
    en_name: String,
    cn_name: String,
    group_name: String,
    level: String,
    ext_info: HashMap<String, String>,
}