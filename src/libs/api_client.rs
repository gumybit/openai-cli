use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

const HOST: &str = "https://api.openai.com/";
const VERSION: &str = "v1";

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChat {
    pub choices: Vec<ResponseChatChoice>,
    id: String,
    usage: ResponseUsage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCompletion {
    pub choices: Vec<ResponseComletionChoice>,
    id: String,
    usage: ResponseUsage,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseUsage {
    prompt_tokens: u16,
    completion_tokens: u16,
    total_tokens: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChatChoice {
    pub message: ResponseChatMessage,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseComletionChoice {
    pub text: String,
    index: u8,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChatMessage {
    role: String,
    pub content: String,
}

fn get_client() -> Client {
    reqwest::Client::new()
}

// ref: https://help.openai.com/en/articles/6654000-best-practices-for-prompt-engineering-with-openai-api

pub struct PostChatArgs {
    pub token: String,
    pub message: String,
    pub role: String,
    pub model: Option<String>,
    pub max_tokens: Option<u16>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<u16>,
    pub user: Option<String>,
}

/// POST /chat/completion
pub async fn post_chat(args: PostChatArgs) -> Result<Response, reqwest::Error> {
    let post_body = json!({
        "model": args.model,
        "messages": vec![json!({"role": args.role, "content": args.message})],
        "temperature": args.temperature,
        "max_tokens": args.max_tokens,
        "top_p": args.top_p,
        "n": args.n,
        "user": args.user
    });

    self::get_client()
        .post(format!(
            "{}{}{}",
            self::HOST,
            self::VERSION,
            "/chat/completions"
        ))
        .bearer_auth(args.token)
        .json(&post_body)
        .send()
        .await
}

pub struct PostComletionArgs {
    pub token: String,
    pub prompt: String,
    pub model: Option<String>,
    pub suffix: Option<String>,
    pub max_tokens: Option<u16>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<u16>,
}

/// POST /completion
pub async fn post_completion(args: PostComletionArgs) -> Result<Response, reqwest::Error> {
    let post_body = json!({
        "model": args.model,
        "prompt": args.prompt,
        "suffix": args.suffix,
        "temperature": args.temperature,
        "top_p": args.top_p,
        "n": args.n,
        "max_tokens": match args.max_tokens { None => 1000, _ => args.max_tokens.unwrap() }
    });

    self::get_client()
        .post(format!("{}{}{}", self::HOST, self::VERSION, "/completions"))
        .bearer_auth(args.token)
        .json(&post_body)
        .send()
        .await
}
