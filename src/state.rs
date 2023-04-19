use std::env;

use crate::{
    config::{ConfigTrait, OpenAIConfig},
    prompt::prompt_user,
    CLICommand,
};

#[derive(Debug)]
pub struct State {
    pub config: OpenAIConfig,
    pub command_log: Vec<CLICommand>,
    pub prompts_log: Vec<String>,
    pub shell: String,
    pub os: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            config: OpenAIConfig::new(),
            command_log: Vec::new(),
            prompts_log: Vec::new(),
            shell: get_shell(),
            os: env::consts::OS.to_string(),
        }
    }
}

fn get_shell() -> String {
    let shell = env::var("SHELL").unwrap();
    let shell = shell.split('/').last().unwrap();
    shell.to_string()
}
