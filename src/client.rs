use std::sync::Arc;
use hyper::Client as HyperClient;
use hyper::client::HttpConnector;
use hyper_openssl::HttpsConnector;

pub(crate) type HttpsHyperClient = HyperClient<HttpsConnector<HttpConnector>>;

#[derive(Debug)]
pub struct Client {
    pub(crate) api_key: String,
    pub(crate) https: HttpsHyperClient,
}