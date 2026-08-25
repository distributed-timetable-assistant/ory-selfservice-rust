use http::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("Http Client Call -> FAILED: Internal error. from_error=({0})")]
    HttpClient(#[from] reqwest_middleware::Error),
    #[error("Identity Service -> FAILED: from_error=({0})")]
    Identity(#[from] IdentityError),
    #[error("OIDC Service -> FAILED: from_error=({0})")]
    Oidc(#[from] OidcError),
    #[error("Extracting Login Challenge -> FAILED: {0}")]
    LoginChallenge(String),
    #[error("Extracting Consent Challenge -> FAILED: {0}")]
    ConsentChallenge(String),
    #[error("Extracting Subject -> FAILED: {0}")]
    Subject(String),
    #[error("Parsing Header Value -> FAILED: source_error=({0})")]
    HeaderValueParse(#[from] InvalidHeaderValue)
}
pub type FlowResult<T> = Result<T, FlowError>;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Flow ID Check -> FAILED: flow_id=({0})")]
    IncorrectFlowIdFormat(String),
    #[error("Error ID Check -> FAILED: error_id=({0})")]
    IncorrectErrorIdFormat(String),
    #[error("Http Middleware Call -> FAILED: Internal error. source_error=({0})")]
    HttpMiddleware(#[from] reqwest_middleware::Error),
    #[error("Http Client Call -> FAILED: Internal error. source_error=({0})")]
    HttpClient(#[from] reqwest::Error),
    #[error("Fetch Identity Flow -> FAILED: {0}")]
    RetrieveRequest(String),
    #[error("Submit Identity Flow -> FAILED: {0}")]
    PostFlow(String),
    #[error("Flow Deserialize -> FAILED: source_error=({0})")]
    FlowDeserialize(#[from] serde_json::Error),
    #[error("Identity Request -> FAILED: Unauthorized. {0}")]
    Unauthorized(String),
    #[error("Check Session -> FAILED: {0}")]
    CheckSession(String),
    #[error("Retrieve Error -> FAILED: {0}")]
    RetrieveError(String),
    #[error("Retrieve Logout -> FAILED: {0}")]
    RetrieveLogout(String),
}
pub type IdentityResult<T> = Result<T, IdentityError>;

#[derive(Error, Debug)]
pub enum OidcError {
    #[error("Http Client Call -> FAILED: Internal error. source_error=({0})")]
    HttpClient(#[from] reqwest::Error),
    #[error("Http Client Call -> FAILED: Internal error. source_error=({0})")]
    HttpMiddleware(#[from] reqwest_middleware::Error),
    #[error("Retrieve OIDC Request -> FAILED: {0}")]
    RetrieveRequest(String),
    #[error("Accept OIDC Request -> FAILED: {0}")]
    AcceptRequest(String),
}
pub type OidcResult<T> = Result<T, OidcError>;
