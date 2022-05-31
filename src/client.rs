use hyper::Client as HyperClient;
use hyper::client::HttpConnector;
use hyper_openssl::HttpsConnector;
use crate::endpoints::{Response, ResponseError};
use crate::endpoints::request::Endpoint;

pub(crate) type HttpsHyperClient = HyperClient<HttpsConnector<HttpConnector>>;

#[derive(Debug)]
pub struct Client {
    pub(crate) api_key: String,
    pub(crate) https: HttpsHyperClient,
}

impl Client {
    /// Returns a new response from the OpenAI API.
    ///
    /// # Arguments
    ///
    /// * `engine_id` - The engine id to use. Due to few endpoints this can be optional.
    /// * `model` - The model to use. Each Model in the endpoints module is a corresponding model.
    ///
    /// # Example
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use openai_rs::client::Client;
    /// use openai_rs::endpoints::edits::Edit;
    /// use openai_rs::endpoints::{Response, ResponseError};
    /// use openai_rs::openai;
    ///
    /// // Create the Client with your API key.
    /// let client: Client = openai::new("api_key");
    ///
    /// // Create the Edit struct with the input and instruction.
    /// let edit = Edit {
    ///      input: Cow::Borrowed("What day of the wek is it?"),
    ///      instruction: Cow::Borrowed("Fix the spelling mistakes"),
    ///      ..Default::default()
    ///  };
    ///
    /// // Send the request to the OpenAI API.
    /// let response: Result<Response, ResponseError> = client.create(
    ///     Some("text-davinci-edit-001"), &edit
    /// ).await;
    /// ```
    pub async fn create<T>(
        &self,
        engine_id: Option<&str>,
        model: &T
    ) -> Result<Response, ResponseError>
        where T: Endpoint {
        match self.https.request(model.request(&*self.api_key, engine_id)).await {
            Ok(response) => {
                if response.status().is_success() {
                    let body = hyper::body::to_bytes(response.into_body()).await?;
                    let deserialized = serde_json::from_slice(&body)
                        .map_err(ResponseError::from)?;
                    trace!("Requesting: {:#?}", deserialized);

                    Ok(deserialized)
                } else {
                    Err(ResponseError::ErrorCode(response.status()))
                }
            },
            Err(error) => Err(error.into())
        }
    }
}