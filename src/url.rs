use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ResponseMode {
    Query,
    Fragment,
    #[default]
    FormPost,
}

impl fmt::Display for ResponseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseMode::Query => write!(f, "query"),
            ResponseMode::Fragment => write!(f, "fragment"),
            ResponseMode::FormPost => write!(f, "form_post"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ResponseType {
    Code,
    #[default]
    CodeId,
}

impl fmt::Display for ResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseType::Code => write!(f, "code"),
            ResponseType::CodeId => write!(f, "code id_token"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeURLConfig {
    pub client_id: String,
    pub redirect_uri: String,
    pub state: Option<String>,
    pub scope: Option<Vec<String>>,
    pub nonce: Option<String>,
    pub response_mode: Option<ResponseMode>,
    pub response_type: Option<ResponseType>,
}

pub fn authorize_url(cfg: AuthorizeURLConfig) -> String {
    let mut url = Url::parse("https://appleid.apple.com/auth/authorize").unwrap();
    let mut query = url.query_pairs_mut();

    query.append_pair(
        "response_type",
        &cfg.response_type.unwrap_or_default().to_string(),
    );
    query.append_pair(
        "response_mode",
        &cfg.response_mode.unwrap_or_default().to_string(),
    );
    query.append_pair("client_id", &cfg.client_id);
    query.append_pair("redirect_uri", &cfg.redirect_uri);

    if let Some(state) = cfg.state {
        query.append_pair("state", &state);
    }
    if let Some(nonce) = cfg.nonce {
        query.append_pair("nonce", &nonce);
    }
    if let Some(scope) = cfg.scope {
        query.append_pair("scope", &scope.join(" "));
    }

    drop(query);
    url.to_string()
}
