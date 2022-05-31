use std::borrow::Cow;
use std::collections::HashMap;
use hyper::{Body, Request};
use serde::Serialize;
use crate::endpoints::Model;
use crate::endpoints::request::Endpoint;

/// Given a question, a set of documents, and some examples, the API generates an answer to the
/// question based on the information in the set of documents.
/// This is useful for question-answering applications on sources of truth,
/// like company documentation or a knowledge base.
#[derive(Debug, Clone, Serialize)]
pub struct Answer<'a> {
    pub model: Model,
    pub question: Cow<'a, str>,
    pub examples: Vec<[Cow<'a, str>; 2]>,
    pub examples_context: Cow<'a, str>,
    pub documents: Vec<Cow<'a, str>>,
    pub file: Option<Cow<'a, str>>,
    pub search_model: Model,
    pub max_rerank: u32,
    pub temperature: f32,
    pub logprobs: u32,
    pub max_tokens: u32,
    pub stop: Option<[char; 4]>,
    pub n: u32,
    pub logit_bias: HashMap<Cow<'a, str>, i32>,
    pub return_metadata: bool,
    pub return_prompt: bool,
    pub expand: Vec<Cow<'a, str>>,
    pub user: Cow<'a, str>
}

impl Default for Answer<'_> {
    fn default() -> Self {
        Self {
            model: Model::Ada,
            question: Cow::Borrowed(""),
            examples: Vec::new(),
            examples_context: Cow::Borrowed(""),
            documents: Vec::new(),
            file: None,
            search_model: Model::Ada,
            max_rerank: 200,
            temperature: 0.0,
            logprobs: 0,
            max_tokens: 16,
            stop: None,
            n: 1,
            logit_bias: HashMap::new(),
            return_metadata: false,
            return_prompt: false,
            expand: Vec::new(),
            user: Cow::Borrowed("")
        }
    }
}

impl Endpoint for Answer<'_> {
    const ENDPOINT: &'static str = "https://api.openai.com/v1/answers";

    fn request(&self, auth_token: &str, _engine_id: Option<&str>) -> Request<Body> {
        let serialized = serde_json::to_string(&self)
            .expect("Failed to serialize Answer");
        let endpoint = Self::ENDPOINT.to_owned();

        super::request::post!(endpoint, auth_token, serialized)
    }
}