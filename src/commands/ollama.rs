// src/ollama.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

pub struct OllamaClient {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaClient {
    pub fn new(base_url: Option<String>, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
            model: model.unwrap_or_else(|| "gemma3:4b".to_string()),
        }
    }

    pub async fn analyze_log(&self, log_content: &str, query: &str, num_lines: usize) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            r#"Du bist ein erfahrener Systemadministrator. Analysiere den folgenden Ausschnitt aus einer Log-Datei.
Beantworte die Frage des Benutzers präzise und kurz. Gib wenn möglich konkrete Beispiele aus dem Log an.

Log-Daten ({} Zeilen):
---
{}
---

Frage des Benutzers: {}"#,
            num_lines, log_content, query
        );

        let request = OllamaRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            stream: false,
        };

        let url = format!("{}/api/chat", self.base_url);

        println!("🤖 Analysiere Log-Daten mit {} ... bitte warten.", self.model);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Ollama API Fehler: {}", response.status()).into());
        }

        let ollama_response: OllamaResponse = response.json().await?;
        Ok(ollama_response.message.content)
    }

    pub async fn check_ollama_availability(&self) -> bool {
        let url = format!("{}/api/tags", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    pub async fn list_available_models(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/tags", self.base_url);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err("Konnte Modelle nicht abrufen".into());
        }

        let data: HashMap<String, serde_json::Value> = response.json().await?;

        let models = data.get("models")
            .and_then(|m| m.as_array())
            .map(|models_array| {
                models_array.iter()
                    .filter_map(|model| model.get("name"))
                    .filter_map(|name| name.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        Ok(models)
    }
}