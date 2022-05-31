use std::fmt::{Debug, Display, Formatter};
use std::io::Error;

pub mod completion;
pub mod classification;
pub mod answer;
pub mod search;
pub mod edits;

use serde::{Deserialize, Serialize};

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
            engine_id: Option<&str>
        ) -> Request<Body>;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub data: Option<Vec<Data>>,
    pub completion: Option<String>,
    pub label: Option<String>,
    pub search_model: Option<Model>,
    pub selected_examples: Option<Vec<SelectedExample>>,
    pub selected_documents: Option<Vec<SelectedDocument>>
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<u32>,
    pub finish_reason: Option<String>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Data {
    pub document: u32,
    pub object: String,
    pub score: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SelectedExample {
    pub document: u32,
    pub label: String,
    pub text: String
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SelectedDocument {
    pub document: u32,
    pub text: String
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Model {
    Ada,
    Babbage,
    Curie,
    Davinci
}

#[derive(Debug)]
pub enum ResponseError {
    Io(Error),
    Hyper(hyper::Error),
    ErrorCode(hyper::StatusCode),
    Serialization(serde_json::Error),
}

impl Default for Model {
    fn default() -> Self {
        Self::Ada
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