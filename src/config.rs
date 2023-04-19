use core::panic;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use serde_derive::{Deserialize, Serialize};

pub trait ConfigTrait {
    fn new() -> Self;
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
    fn list(&self);
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenAIConfig {
    pub openai_api_key: String,
    pub model: String,
    pub temperature: f64,
    pub n: u32,
    pub stream: bool,
}

#[derive(Deserialize)]
struct Config {
    open_ai: OpenAIConfig,
}

impl ConfigTrait for OpenAIConfig {
    fn new() -> Self {
        let config_path = PathBuf::from(format!("{}/config.toml", env::var("HOME").unwrap()));
        // check if config file exists
        if !config_path.exists() {
            // create config file
            let config_toml = toml::to_string(&Self {
                openai_api_key: "".to_owned(),
                model: "gpt-3.5-turbo".to_owned(),
                temperature: 0.4,
                n: 1,
                stream: false,
            })
            .expect("Failed to serialize OpenAIConfig to TOML");
            let final_conf = format!("[open_ai]\n{}\n", config_toml);
            fs::write(&config_path, final_conf).expect("Failed to write to config file");
        }
        let config_toml = fs::read_to_string(&config_path).expect("Failed to read config file");
        let config: Config = toml::from_str(&config_toml).expect("Failed to parse TOML");
        let openai_config = config.open_ai;

        Self {
            openai_api_key: openai_config.openai_api_key,
            model: openai_config.model,
            temperature: openai_config.temperature,
            n: openai_config.n,
            stream: openai_config.stream,
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        match key {
            "model" => Some(self.model.to_owned()),
            "temperature" => Some(self.temperature.to_string()),
            "n" => Some(self.n.to_string()),
            "stream" => Some(self.stream.to_string()),
            _ => None,
        }
    }

    fn set(&mut self, key: &str, value: &str) {
        match key {
            "model" => self.model = value.to_owned(),
            "temperature" => self.temperature = value.parse().unwrap(),
            "n" => self.n = value.parse().unwrap(),
            "stream" => self.stream = value.parse().unwrap(),
            "openai_api_key" => self.openai_api_key = value.to_owned(),
            _ => (),
        }

        // Update the config.toml file
        let config_toml = toml::to_string(&self).expect("Failed to serialize OpenAIConfig to TOML");
        let config_path = PathBuf::from(format!("{}/config.toml", env::var("HOME").unwrap()));
        let final_conf = format!("[open_ai]\n{}\n", config_toml);
        fs::write(&config_path, &final_conf).expect("Failed to write to config file");
    }

    fn list(&self) {
        // show your current settings
        println!(
            "{:?}",
            vec![
                format!("model: {}", self.model),
                format!("temperature: {}", self.temperature),
                format!("n: {}", self.n),
                format!("stream: {}", self.stream),
            ]
        );
    }
}
