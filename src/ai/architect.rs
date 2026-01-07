#![allow(dead_code)]
use std::env;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[allow(dead_code)]
pub struct Architect {
    client: Client,
    api_key: String,
    api_url: String,
}

impl Architect {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let api_key = env::var("ARCHITECT_API_KEY").unwrap_or_else(|_| "stub_key".to_string());
        let api_url = env::var("ARCHITECT_API_URL").unwrap_or_else(|_| "https://api.openai.com/v1/completions".to_string());
        
        Ok(Self {
            client: Client::new(),
            api_key,
            api_url,
        })
    }

    pub async fn generate_persona(&self, archetype: &str) -> Result<String, Box<dyn Error>> {
        if self.api_key == "stub_key" {
            return Ok(format!("Stub Persona: suspicious {} administrator", archetype));
        }

        let prompt = format!(
            "Generate a specialized system persona for a '{}' in a cyberpunk kernel architecture. 
            Response should be a short JSON object description.", 
            archetype
        );

        let request = CompletionRequest {
            model: "text-curie-001".to_string(), // Using cheaper legacy model as requested or update to gpt-3.5-turbo-instruct
            prompt,
            max_tokens: 100,
            temperature: 0.8,
        };

        let resp = self.client.post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<CompletionResponse>()
            .await?;

        Ok(resp.choices.get(0).map(|c| c.text.clone()).unwrap_or_default())
    }
}
