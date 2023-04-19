#![allow(dead_code, unused)]

use inquire::{InquireError, Select};
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};

use crate::{
    config::{ConfigTrait, OpenAIConfig},
    prompt::prompt_user,
    state::State,
};
#[derive(Debug)]
pub enum Prompt {
    Command(String, String, String),
    Revision(String, String, String, String),
    Explain(String),
}

const REVISION_PROMPT: &str =
    "Update the following command based on what is asked in the following prompt";
const EXPLAIN_PROMPT: &str =
    "Describe the command in plain english, step by step, what exactly it does.

Please describe succinctly, use as few words as possible, do not be verbose.
If there are multiple steps, display them as a list.";

impl Prompt {
    pub fn value(&self) -> String {
        match self {
            Prompt::Command(shell, os, prompt) => format!(
                "I will give you a prompt to create a single line command that one can enter in a terminal and run, based on what is asked in the prompt.\
                \nThe target terminal is {}.\
                \nPlease only reply with the single line command in plain text without Markdown formatting. It should be able to be directly run in a terminal. Do not include any other text.\
                \nPlease make sure the script runs on {} operating system.\
                \nPrompt: {}",
                shell, os, prompt
            ),
            Prompt::Revision(shell, os, prompt, command) => format!(
                "{}\
                \nThe target terminal is {}.\
                \nPlease only reply with the single line command in plain text without Markdown formatting. It should be able to be directly run in a terminal. Do not include any other text.\
                \nPlease make sure the script runs on {} operating system.\
                \nCommand: {}\
                \nPrompt: {}",
                REVISION_PROMPT, shell, os, command, prompt
            ),
            Prompt::Explain(command) => format!(
                "{}

                This is the script: {}",
                EXPLAIN_PROMPT, command
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
    n: u32,
    stream: bool,
}

pub async fn get_completion(config: OpenAIConfig, message: Prompt) -> ChatCompletionResponse {
    let request_body = RequestBody {
        model: config.model.to_owned(),
        messages: vec![Message {
            role: "user".to_owned(),
            content: message.value(),
        }],
        temperature: config.temperature,
        n: config.n,
        stream: config.stream,
    };

    let request_body_json = serde_json::to_string(&request_body).unwrap();

    let response = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .body(request_body_json)
        .send()
        .await;

    match response {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => {
                let response_str = response.text().await.unwrap();

                serde_json::from_str::<ChatCompletionResponse>(&response_str)
                    .expect("Failed to parse response from OpenAI API")
            }
            _ => {
                let status = response.status();
                let error_message = response.text().await.unwrap();
                let error_details = serde_json::from_str::<serde_json::Value>(&error_message)
                    .ok()
                    .and_then(|json| {
                        let error = json.get("error")?;
                        let message = error.get("message")?.as_str()?.to_string();
                        let code = error
                            .get("code")
                            .map(|c| c.as_str().unwrap_or("unknown"))
                            .unwrap_or("unknown");
                        Some((message, code.to_string()))
                    });
                if let Some((message, code)) = error_details {
                    eprintln!("Error: {} - {}", status, message);
                } else {
                    eprintln!("Error: {} - {}", status, error_message);
                }
                std::process::exit(1);
            }
        },
        Err(error) => {
            panic!("Error: {}", error)
        }
    }
}

pub async fn get_command(prompt: String, state: &mut State) -> String {
    let mut chat_completion_res: ChatCompletionResponse;
    if !state.command_log.is_empty() {
        let prev_command = state.command_log.last().unwrap().clone();
        chat_completion_res = get_completion(
            OpenAIConfig::new(),
            Prompt::Revision(state.shell.clone(), state.os.clone(), prompt, prev_command),
        )
        .await;
    } else {
        chat_completion_res = get_completion(
            OpenAIConfig::new(),
            Prompt::Command(state.os.clone(), state.shell.clone(), prompt),
        )
        .await;
    }

    get_cli_command_from_prompt(chat_completion_res.choices[0].message.content.as_str())
        .expect("Failed to get command from prompt")
}

pub async fn get_explanation(command: String) -> String {
    let chat_completion_res = get_completion(OpenAIConfig::new(), Prompt::Explain(command)).await;
    chat_completion_res.choices[0].message.content.to_owned()
}

fn get_cli_command_from_prompt(prompt: &str) -> Result<String, String> {
    let command = prompt
        .trim()
        .trim_start_matches("```")
        .trim_end_matches("```")
        .replace('\n', "");

    if command.is_empty() {
        return Err("No command found in prompt".to_owned());
    }
    Ok(command)
}
