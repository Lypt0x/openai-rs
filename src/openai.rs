use crate::client::Client;

pub fn new(api_key: &str) -> Client {
    Client { api_key: api_key.to_owned() }
}