use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Awareness {
    system_type: String,
    memory: Vec<String>,
    parameters: HashMap<String, String>
}