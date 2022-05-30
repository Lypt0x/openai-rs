use std::fmt::{Debug, Display, Formatter};
use std::io::Error;

pub mod completion;
pub mod classification;
pub mod code;
pub mod embeddings;
pub mod finetuning;
pub mod qa;
pub mod search;
pub mod edits;

use serde::Deserialize;

/// This request-Module is for internal purpose
pub(crate) mod request {
    use hyper::{Body, Request};
    use serde::Serialize;

    macro_rules! post {
        ($endpoint:ident, $auth_token:ident, $serialized:ident) => {{
            hyper::http::Request::builder()
                .method(hyper::http::method::Method::POST)
                .uri($endpoint)
                .header(hyper::header::AUTHORIZATION, &format!("Bearer {}", $auth_token))
                .header(hyper::header::CONTENT_TYPE, "application/json")
                .body(hyper::body::Body::from($serialized)).expect("Failed to build request")
        }}
    }
    pub(super) use post;

    /// An Endpoint-Trait which contains the ability to form a request.
    /// This trait is mainly used for internal purpose (implementation of the Endpoint-Trait)
    pub trait Endpoint
    where Self: Serialize {
        const ENDPOINT: &'static str;

        fn request(
            &self,
            auth_token: &str,
            engine_id: &str
        ) -> Request<Body>;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<u32>,
    pub finish_reason: String
}

#[derive(Debug)]
pub enum ResponseError {
    Io(Error),
    Hyper(hyper::Error),
    ErrorCode(hyper::StatusCode),
    Serialization(serde_json::Error),
}

impl Response {
    pub fn choice_response(&self) -> Option<&str> {
        Some(self.choices.first()?.text.as_str())
    }
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::Io(error) => write!(f, "IO error: {}", error),
            ResponseError::Hyper(error) => write!(f, "Hyper error: {}", error),
            ResponseError::ErrorCode(status) => write!(f, "Error code: {}", status),
            ResponseError::Serialization(error) => write!(f, "Serialization error: {}", error),
        }
    }
}

impl From<serde_json::Error> for ResponseError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}

impl From<Error> for ResponseError {
    fn from(error: Error) -> Self {
        Self::Io(error)
    }
}

impl From<hyper::Error> for ResponseError {
    fn from(error: hyper::Error) -> Self {
        Self::Hyper(error)
    }
}

impl std::error::Error for ResponseError {}