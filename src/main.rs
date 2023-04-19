#![allow(dead_code, unused)]

mod completion;
mod config;
mod prompt;
mod state;

use std::{
    io::{self, stdout, Read, Write},
    process::Command,
    thread::{self, sleep},
    time::Duration,
};

use clap::Parser;
use completion::{get_command, Prompt};
use config::OpenAIConfig;
use inquire::Text;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use state::State;
use termion::{
    color::Fg,
    style::{Bold, Reset},
};

use crate::{
    completion::{get_completion, get_explanation},
    config::ConfigTrait,
    prompt::{prompt_user, UserOption},
};

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(name = "set")]
    Set(SetCmd),
}

#[derive(Parser, Debug)]
struct SetCmd {
    #[clap(name = "key=value", help = "Key value pair to set.")]
    key_value: Vec<String>,
}

#[derive(Parser, Debug)]
#[clap(name = "cli-pilot", about = "Your rust personal cli assitant.")]
struct Args {
    #[clap(subcommand)]
    cmd: Option<SubCommand>,
    #[clap(name = "PROMPT", help = "The prompt to use.")]
    prompt: Vec<String>,
}
type CLICommand = String;
type Explanation = String;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let mut args = Args::parse();
    let mut cur_prompt = "".to_string();
    if let Some(cmd) = args.cmd {
        match cmd {
            SubCommand::Set(set_cmd) => {
                let mut config = OpenAIConfig::new();
                if (set_cmd.key_value.len() > 1) {
                    args.prompt = set_cmd.key_value.clone();
                    cur_prompt = "set".to_string();
                } else {
                    let key_value: Vec<&str> = set_cmd.key_value[0].split("=").collect();
                    config.set(key_value[0].clone(), key_value[1].clone());
                    println!("Updated config successfully.");
                    std::process::exit(0);
                    return Ok(());
                }
            }
        }
    }

    if args.prompt.is_empty() {
        eprintln!("Error: Please provide a prompt");
        std::process::exit(1);
    }

    let mut state = State::new();
    cur_prompt = format!("{} {}", &cur_prompt, args.prompt.join(" "));
    state.prompts_log.push(cur_prompt.clone());

    loop {
        if !state.command_log.is_empty() {
            let revision_prompt = Text::new("Enter revision: ").prompt();
            cur_prompt = match revision_prompt {
                Ok(revision_prompt) => {
                    state.prompts_log.push(revision_prompt.clone());
                    revision_prompt
                }
                Err(error) => {
                    eprintln!("{}", error);
                    continue;
                }
            };
        }
        println!();

        if state.prompts_log.len() > 1 {
            println!(
                "{}{}--------Query---------{}",
                Bold,
                Fg(termion::color::Cyan),
                Reset
            );
            for (idx, prompt) in state.prompts_log.iter().enumerate() {
                println!("{}{}) {}{}", Bold, idx + 1, prompt, Reset);
            }
        }
        println!();

        println!(
            "{}{}--------Command---------{}",
            Bold,
            Fg(termion::color::Cyan),
            Reset
        );
        println!();
        let cli_command = request_command(cur_prompt.clone(), &mut state).await;
        let command_str = match cli_command {
            Ok(cli_command) => cli_command,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        };
        println!();

        println!(
            "{}{}--------Explanation---------{}",
            Bold,
            Fg(termion::color::Cyan),
            Reset
        );
        println!();
        let explanation = request_explanation(command_str.clone()).await;
        let explanatino_str = match explanation {
            Ok(explanation) => explanation,
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        };
        println!();

        menu(command_str.clone(), &mut state).await;
    }
}

async fn request_command(prompt: String, state: &mut State) -> Result<CLICommand, String> {
    let mut sp = Spinner::new(Spinners::Dots, "Talking to AI 🤖".into());
    let command = get_command(prompt.clone(), state).await;
    state.command_log.push(command.clone());
    sp.stop_with_message(command.clone());

    Ok(command)
}

async fn request_explanation(command: String) -> Result<Explanation, String> {
    let mut sp = Spinner::new(Spinners::Dots, "Talking to AI 🤖".into());
    let explanation = get_explanation(command.clone()).await;
    sp.stop_with_message(explanation.clone());
    Ok(explanation)
}

async fn menu(command: String, state: &mut State) {
    let ans = prompt_user();

    match ans {
        Ok(choice) => match UserOption::from_str(choice) {
            Some(UserOption::Run) => {
                println!("Running command: {}", command);
                println!();
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .expect("failed to execute command");

                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                }

                std::process::exit(0);
            }
            Some(UserOption::Revise) => {}
            Some(UserOption::Cancel) => {
                std::process::exit(0);
            }
            None => {
                println!("Unknown choice: {}", choice);
            }
        },
        Err(_) => println!("There was an error, please try again"),
    }
}
