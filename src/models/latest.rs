use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Latest {
    pub disclaimer: String,
    pub license: String,
    pub timestamp: u32,
    pub base: String,
    pub rates: HashMap<String, f32>,
}
