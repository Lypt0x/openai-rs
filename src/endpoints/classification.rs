use std::borrow::Cow;
use std::collections::HashMap;
use hyper::{Body, Request};
use serde::Serialize;
use crate::endpoints::Model;
use crate::endpoints::request::Endpoint;

/// Given a query and a set of labeled examples, the model will predict the most likely label for the query.
/// Useful as a drop-in replacement for any ML classification or text-to-label task.
#[derive(Debug, Clone, Serialize)]
pub struct Classification<'a> {
    /// ID of the engine to use for completion. You can select one of ada, babbage, curie, or davinci.
    pub model: Model,

    /// Query to be classified.
    pub query: Cow<'a, str>,

    /// A list of examples with labels, in the following format:
    /// `[["The movie is so interesting.", "Positive"], ["It is quite boring.", "Negative"], ...]`
    /// All the label strings will be normalized to be capitalized.
    /// You should specify either examples or file, but not both.
    pub examples: Vec<[Cow<'a, str>; 2]>,

    /// The ID of the uploaded file that contains training examples.
    /// See upload file for how to upload a file of the desired format and purpose.
    /// You should specify either examples or file, but not both.
    pub file: Option<Cow<'a, str>>,

    /// The set of categories being classified. If not specified, candidate labels will be
    /// automatically collected from the examples you provide. All the label strings will be
    /// normalized to be capitalized.
    pub labels: Vec<Cow<'a, str>>,

    /// ID of the engine to use for Search. You can select one of ada, babbage, curie, or davinci
    pub search_model: Model,

    /// What sampling temperature to use. Higher values mean the model will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    pub temperature: f32,

    /// Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens.
    /// For example, if logprobs is 5, the API will return a list of the 5 most likely tokens.
    /// The API will always return the logprob of the sampled token,
    /// so there may be up to logprobs+1 elements in the response.
    /// The maximum value for logprobs is 5.
    /// If you need more than this, please contact support@openai.com and describe your use case.
    pub logprobs: u32,

    /// The maximum number of examples to be ranked by Search when using file.
    /// Setting it to a higher value leads to improved accuracy but with increased latency and cost.
    pub max_examples: u32,

    /// Modify the likelihood of specified tokens appearing in the completion.
    /// Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer)
    /// to an associated bias value from -100 to 100. You can use this tokenizer tool (which works
    /// for both GPT-2 and GPT-3) to convert text to token IDs. Mathematically,
    /// the bias is added to the logits generated by the model prior to sampling. The exact effect
    /// will vary per model, but values between -1 and 1 should decrease or increase likelihood
    /// of selection; values like -100 or 100 should result in a ban or exclusive selection
    /// of the relevant token.
    pub logit_bias: HashMap<Cow<'a, str>, i32>,

    /// If set to true, the returned JSON will include a "prompt" field containing the final prompt
    /// that was used to request a completion. This is mainly useful for debugging purposes.
    pub return_prompt: bool,

    /// A special boolean flag for showing metadata. If set to true, each document entry in the
    /// returned JSON will contain a "metadata" field. This flag only takes effect when file is set.
    pub return_metadata: bool,

    /// If set to true, the returned JSON will include a "prompt" field containing the final prompt
    /// that was used to request a completion. This is mainly useful for debugging purposes.
    pub expand: Vec<Cow<'a, str>>,

    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
    pub user: Cow<'a, str>,
}

impl Default for Classification<'_> {
    fn default() -> Self {
        Self {
            model: Model::Davinci,
            query: Cow::Borrowed(""),
            examples: vec![],
            file: None,
            labels: vec![],
            search_model: Model::default(),
            temperature: 0.,
            logprobs: 0,
            max_examples: 200,
            logit_bias: HashMap::new(),
            return_prompt: false,
            return_metadata: false,
            expand: Vec::new(),
            user: Cow::Borrowed("")
        }
    }
}

impl Endpoint for Classification<'_> {
    const ENDPOINT: &'static str = "https://api.openai.com/v1/classifications";

    fn request(&self, auth_token: &str, _engine_id: Option<&str>) -> Request<Body> {
        let serialized = serde_json::to_string(self)
            .expect("Failed to serialize Classification");
        let endpoint = Self::ENDPOINT.to_owned();

        super::request::post!(endpoint, auth_token, serialized)
    }
}