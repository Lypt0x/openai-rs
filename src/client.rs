use std::sync::Arc;
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
    pub async fn create<T>(
        &self,
        engine_id: &str,
        model: &T
    ) -> Result<Response, ResponseError>
        where T: Endpoint {
        match self.https.request(model.request(engine_id)).await {
            Ok(response) => {
                if response.status().is_success() {
                    let body = hyper::body::to_bytes(response.into_body()).await?;

                    Ok(serde_json::from_slice(&body).map_err(ResponseError::from)?)
                } else {
                    Err(ResponseError::ErrorCode(response.status()))
                }
            },
            Err(error) => Err(error.into())
        }
    }
}