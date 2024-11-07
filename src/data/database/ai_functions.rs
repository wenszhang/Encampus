use leptos::create_signal;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

pub async fn get_ai_response(text: String) -> Result<String, reqwest::Error> {
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

    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);

    // let openai_response: OpenAIResponse = response.json().await?;
    // Ok(openai_response.choices.unwrap()[0].message.content.clone( ))
    Ok("test".to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiRequest {
    prompt: String,
    model: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiResponse {
    text: String,
}

pub async fn get_gemini_response(input: String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let api_key = "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";

    let request = GeminiRequest {
        model: "gpt-3.5-turbo".to_string(),
        prompt: input,
    };

    let response = client
        .post("url")
        .json(&request)
        .send()
        .await
        .unwrap()
        .json::<GeminiResponse>()
        .await
        .unwrap();

    let response_text = response.text;
    println!("Raw response: {}", response_text);

    Ok("test".to_string())
}
