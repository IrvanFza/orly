use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrlyError {
    #[error("Request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),
    #[error("Failed to parse xml/html: {0}")]
    XmlParseError(#[from] libxml::parser::XmlParseError),
    #[error("Failed to parse xml/html: {0}")]
    ParseError(String),
    #[error("Xpath error")]
    XpathError(()),
    #[error("Authentication failure: {0}")]
    AuthenticationFailed(String),
    #[error("Subscription expired")]
    SubscriptionExpired,
    #[error("Password login is not supported for account {0}")]
    PasswordLoginUnsupported(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = anyhow::Result<T, OrlyError>;
