use std::borrow::Cow;
use hyper::{Body, Request};
use serde::Serialize;
use crate::endpoints::request::Endpoint;

/// Given a query and a set of documents or labels, the model ranks each document based
/// on its semantic similarity to the provided query.
#[derive(Debug, Clone, Serialize)]
pub struct Search<'a> {
    /// Query to search against the documents.
    pub query: Cow<'a, str>,

    /// Up to 200 documents to search over, provided as a list of strings.
    /// The maximum document length (in tokens) is 2034 minus the number of tokens in the query.
    /// You should specify either documents or a file, but not both.
    pub documents: Vec<String>,

    /// The ID of an uploaded file that contains documents to search over.
    /// You should specify either documents or a file, but not both.
    pub file: Option<Cow<'a, str>>,

    /// The maximum number of documents to be re-ranked and returned by search.
    /// This flag only takes effect when file is set.
    pub max_rerank: u32,

    /// A special boolean flag for showing metadata. If set to true, each document entry in
    /// the returned JSON will contain a "metadata" field.
    /// This flag only takes effect when file is set.
    pub return_metadata: bool,

    /// A unique identifier representing your end-user, which will help OpenAI to monitor and detect abuse.
    pub user: Cow<'a, str>
}

impl Default for Search<'_> {
    fn default() -> Self {
        Self {
            query: Cow::Borrowed(""),
            documents: Vec::new(),
            file: None,
            max_rerank: 200,
            return_metadata: false,
            user: Cow::Borrowed("")
        }
    }
}

impl Endpoint for Search<'_> {
    const ENDPOINT: &'static str = "https://api.openai.com/v1/engines/{}/search";

    fn request(&self, auth_token: &str, engine_id: Option<&str>) -> Request<Body> {
        let endpoint = Self::ENDPOINT.replace("{}", engine_id.unwrap());
        let serialized = serde_json::to_string(&self)
            .expect("Failed to serialize Search");

        super::request::post!(endpoint, auth_token, serialized)
    }
}