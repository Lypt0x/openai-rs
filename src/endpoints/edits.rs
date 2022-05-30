use std::borrow::Cow;
use hyper::{Body, Request};
use serde::Serialize;
use crate::endpoints::request::Endpoint;

/// Given a prompt and an instruction, the model will return an edited version of the prompt.
#[derive(Debug, Clone, Serialize)]
pub struct Edit<'a> {
    /// The input text to use as a starting point for the edit.
    pub input: Cow<'a, str>,

    /// The instruction that tells the model how to edit the prompt.
    pub instruction: Cow<'a, str>,

    /// What sampling temperature to use. Higher values means the model will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    /// We generally recommend altering this or top_p but not both.
    pub temperature: f32,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model
    /// considers the results of the tokens with top_p probability mass. So 0.1 means only
    /// the tokens comprising the top 10% probability mass are considered.
    /// We generally recommend altering this or temperature but not both.
    pub top_p: f32,
}

impl Default for Edit<'_> {
    fn default() -> Self {
        Self {
            input: Cow::Borrowed(""),
            instruction: Cow::Borrowed(""),
            temperature: 0.,
            top_p: 0.
        }
    }
}

impl Endpoint for Edit<'_> {
    const ENDPOINT: &'static str = "https://api.openai.com/v1/engines/{}/edits";

    fn request(&self, auth_token: &str, engine_id: Option<&str>) -> Request<Body> {
        let endpoint = Self::ENDPOINT.replace("{}", engine_id.unwrap());
        let serialized = serde_json::to_string(&self)
            .expect("Failed to serialize Edit");

        super::request::post!(endpoint, auth_token, serialized)
    }
}