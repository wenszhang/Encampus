// use axum::response;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Option<Vec<Choice>>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

pub async fn get_openai_response(text: String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let api_key = "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";

    let request = OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: text,
        }],
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    let openai_response: OpenAIResponse = response.json().await?;
    Ok(openai_response.choices.unwrap()[0].message.content.clone())
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Candidate {
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Part {
    text: String,
}

pub async fn get_gemini_response(input: String) -> Result<String> {
    let _project_id = "874592041558";
    let api_key = "AIzaSyC4lMM_E_6ge-6L76YDi1Uj_VspRtKng_U";
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);
    // Ideally use gemini-1.5-flash-latest but hasn't always been reliable

    let client = Client::new();

    let body = json!({
        "contents": [{
            "parts": [{
                "text": input
            }]
        }]
    });

    let request = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;

    let response = client.execute(request).await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        let gemini_response: GeminiResponse = serde_json::from_str(&response_text)?;

        if let Some(first_candidate) = gemini_response.candidates.first() {
            if let Some(first_part) = first_candidate.content.parts.first() {
                return Ok(first_part.text.clone());
            }
        }
        Ok("Response structure unexpected".to_string())
    } else {
        println!("Error: {:?}", response);
        Ok(format!("Request failed with status: {}", response.status()))
    }
}
