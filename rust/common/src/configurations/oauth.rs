use super::utils::get_env_vars_by_prefix;
use serde::Deserialize;
use typed_builder::TypedBuilder;

#[derive(Deserialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "lowercase")]
pub struct OauthGoogleCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl Default for OauthGoogleCredentials {
    fn default() -> Self {
        get_env_vars_by_prefix("OAUTH_GOOGLE_")
    }
}

#[derive(Deserialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "lowercase")]
pub struct OauthGithubCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl Default for OauthGithubCredentials {
    fn default() -> Self {
        get_env_vars_by_prefix("OAUTH_GITHUB_")
    }
}

#[derive(Deserialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "lowercase")]
pub struct OauthCredentials {
    #[serde(flatten)]
    pub google: OauthGoogleCredentials,

    #[serde(flatten)]
    pub github: OauthGithubCredentials,
}

impl Default for OauthCredentials {
    fn default() -> Self {
        get_env_vars_by_prefix("OAUTH_")
    }
}
