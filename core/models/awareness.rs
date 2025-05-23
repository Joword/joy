use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use 

#[derive(Serialize, Deserialize, Clone)]
pub struct Awareness {
    system_type: String,
    memory: Vec<String>,
    parameters: HashMap<String, String>
}

struct DecisionHub{
    models: HashMap<String, String>
    self_awareness: Mutex<Awareness>
    create_time: String
    update_time: String
}

