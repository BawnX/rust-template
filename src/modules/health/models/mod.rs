use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub system_status: bool,
    pub database_status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub health: Health,
}

pub struct ObtainHealth {
    pub system_status: bool,
    pub database_status: bool,
}

impl From<Health> for HealthResponse {
    fn from(health: Health) -> Self {
        HealthResponse {
            health: Health {
                system_status: health.system_status,
                database_status: health.database_status
            },
        }
    }
}
