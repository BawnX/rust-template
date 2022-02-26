use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub system_status: bool,
}

pub struct ObtainHealthQuery {
    pub system_status: RwLock<bool>,
}

impl Default for ObtainHealthQuery {
    fn default() -> Self {
        Self {
            system_status: RwLock::new(false),
        }
    }
}
