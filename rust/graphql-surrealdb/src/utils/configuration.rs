use common::configurations::utils::get_env_vars_by_prefix;
use serde::{Deserialize, Serialize};

// For other configurations e.g github_secret
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ExternalApiConfigs {
    pub github_secret: String,
    pub github_client_id: String,
}

impl ExternalApiConfigs {
    pub fn get() -> ExternalApiConfigs {
        get_env_vars_by_prefix("OTHERS_")
    }
}
