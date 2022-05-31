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
    /// ID of the engine to use for completion. You can select one of ada, babbage, curie, or davinci.
    pub model: Model,

    /// The question to answer.
    pub question: Cow<'a, str>,

    /// A list of documents to use for answering the question.
    pub examples: Vec<[Cow<'a, str>; 2]>,

    /// A text snippet containing the contextual information used to generate the answers
    /// for the examples you provide.
    pub examples_context: Cow<'a, str>,

    /// List of documents from which the answer for the input question should be derived.
    /// If this is an empty list, the question will be answered based on the question-answer examples.
    /// You should specify either documents or a file, but not both.
    pub documents: Vec<Cow<'a, str>>,

    /// The ID of an uploaded file that contains documents to search over.
    /// See upload file for how to upload a file of the desired format and purpose.
    /// You should specify either documents or a file, but not both.
    pub file: Option<Cow<'a, str>>,

    /// ID of the engine to use for Search. You can select one of ada, babbage, curie, or davinci.
    pub search_model: Model,

    /// The maximum number of documents to be ranked by Search when using file.
    /// Setting it to a higher value leads to improved accuracy but with increased latency and cost.
    pub max_rerank: u32,

    /// What sampling temperature to use. Higher values mean the model will take more risks
    /// and value 0 (argmax sampling) works better for scenarios with a well-defined answer.
    pub temperature: f32,

    /// Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens.
    /// For example, if logprobs is 5, the API will return a list of the 5 most likely tokens.
    /// The API will always return the logprob of the sampled token,
    /// so there may be up to logprobs+1 elements in the response.
    /// The maximum value for logprobs is 5.
    /// If you need more than this, please contact support@openai.com and describe your use case.
    pub logprobs: u32,

    /// The maximum number of tokens allowed for the generated answer
    pub max_tokens: u32,

    /// Up to 4 sequences where the API will stop generating further tokens.
    /// The returned text will not contain the stop sequence.
    pub stop: Option<[char; 4]>,

    /// How many answers to generate for each question.
    pub n: u32,

    /// Modify the likelihood of specified tokens appearing in the completion.
    /// Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer)
    /// to an associated bias value from -100 to 100. You can use this tokenizer tool (which works
    /// for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically,
    /// the bias is added to the logits generated by the model prior to sampling. The exact effect
    /// will vary per model, but values between -1 and 1 should decrease or increase likelihood
    /// of selection; values like -100 or 100 should result in a ban or exclusive selection
    /// of the relevant token.
    pub logit_bias: HashMap<Cow<'a, str>, i32>,

    /// A special boolean flag for showing metadata. If set to true, each document entry in the
    /// returned JSON will contain a "metadata" field. This flag only takes effect when file is set.
    pub return_metadata: bool,

    /// If set to true, the returned JSON will include a "prompt" field containing the final prompt
    /// that was used to request a completion. This is mainly useful for debugging purposes.
    pub return_prompt: bool,

    /// If an object name is in the list, we provide the full information of the object;
    /// otherwise, we only provide the object ID. Currently we support completion and file objects for expansion.
    pub expand: Vec<Cow<'a, str>>,

    /// A unique identifier representing your end-user,
    /// which will help OpenAI to monitor and detect abuse.
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