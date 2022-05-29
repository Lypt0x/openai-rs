use std::fmt::{Debug, Display, Formatter};
use std::io::Error;

pub mod completion;
pub mod classification;
pub mod code;
pub mod embeddings;
pub mod finetuning;
pub mod qa;
pub mod search;

pub trait Endpoint {
    const ENDPOINT: &'static str;

    fn request(&self, engine_id: &str) -> Result<Response, ResponseError>;
}

pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>
}

pub struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<u32>,
    pub finish_reason: String
}

#[derive(Debug)]
pub enum ResponseError {
    Io(Error),
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::Io(error) => write!(f, "IO error: {}", error),
        }
    }
}

impl From<Error> for ResponseError {
    fn from(error: Error) -> Self {
        Self::Io(error)
    }
}

impl std::error::Error for ResponseError {}