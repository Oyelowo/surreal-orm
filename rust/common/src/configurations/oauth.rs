use super::utils::get_env_vars_by_prefix;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OauthGoogleConfigs {
    pub client_id: String,
    pub client_secret: String,
}

impl Default for OauthGoogleConfigs {
    fn default() -> Self {
        get_env_vars_by_prefix("OAUTH_GOOGLE_")
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OauthGithubConfigs {
    pub client_id: String,
    pub client_secret: String,
}

impl Default for OauthGithubConfigs {
    fn default() -> Self {
        get_env_vars_by_prefix("OAUTH_GITHUB_")
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OauthConfigs {
    #[serde(flatten)]
    pub google: OauthGoogleConfigs,

    #[serde(flatten)]
    pub github: OauthGithubConfigs,
}

impl Default for OauthConfigs {
    fn default() -> Self {
        get_env_vars_by_prefix("OAUTH_")
    }
}
