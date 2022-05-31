# openai-rs
OpenAI library made in Rust

 The openai-rs crate is a Rust library for the OpenAI API.

 The crate is a wrapper around the OpenAI API. It provides a client for the API and a set of
 endpoints for almost each API endpoint.

 # Quick Start
 ```rust
 use std::borrow::Cow;
 use openai_rs::client::Client;
 use openai_rs::endpoints::edits::Edit;
 use openai_rs::endpoints::{Response, ResponseError};
 use openai_rs::openai;

 #[tokio::main]
 async fn main() {
     // Create the Client with your API key.
     let client: Client = openai::new("api_key");

     // Create the Edit struct with the input and instruction.
     let edit = Edit {
         input: Cow::Borrowed("What day of the wek is it?"),
         instruction: Cow::Borrowed("Fix the spelling mistakes"),
         ..Default::default()
     };

     // Send the request to the OpenAI API.
     let response: Result<Response, ResponseError> = client.create(
         Some("text-davinci-edit-001"), &edit
     ).await;
 }
 ```

 # Requirements
 * An api key at [OpenAI API](https://beta.openai.com/docs/introduction) for the Client.
 * An async runtime like [tokio](https://tokio.rs) in order to use the async functions.
