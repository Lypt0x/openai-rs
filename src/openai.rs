use hyper_openssl::HttpsConnector;
use crate::client::Client;
use crate::endpoints::{Response, ResponseError};
use crate::endpoints::request::Endpoint;

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

                    Ok(serde_json::from_slice(&body).map_err::<ResponseError, _>(
                            |error| error.into()
                    )?)
                } else {
                    Err(ResponseError::ErrorCode(response.status()))
                }
            },
            Err(error) => Err(error.into())
        }
    }
}