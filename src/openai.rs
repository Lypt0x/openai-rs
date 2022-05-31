use hyper_openssl::HttpsConnector;
use crate::client::Client;

/// Returns a new client for the OpenAI API.
///
/// # Arguments
/// * `api_key` - The API key to use.
///
/// # Example
/// ```
/// use openai_rs::client::Client;
/// use openai_rs::openai;
///
/// // Create the Client with your API key.
/// let client: Client = openai::new("api_key");
/// ```
pub fn new(api_key: &str) -> Client {
    let hyper_client = hyper::Client::builder()
        .http2_only(true)
        .pool_idle_timeout(std::time::Duration::from_secs(10))
        .build(HttpsConnector::new().expect("Could not create HTTPS connector"));

    Client {
        api_key: api_key.to_owned(),
        https: hyper_client,
    }
}