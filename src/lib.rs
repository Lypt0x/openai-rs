pub mod openai;
pub mod client;
pub mod endpoints;

#[macro_use]
extern crate log;
extern crate core;

#[cfg(test)]
mod tests {
    use crate::endpoints::completion::Completion;
    use crate::openai;

    pub fn not_used() {

        let client = openai::new("api_key");
        let completion = Completion::default();

        let response = client.create(
            "engine_id",
            &completion
        );

    }

}
