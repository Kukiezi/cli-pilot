# CLI-Pilot

CLI-Pilot is a Rust-based personal CLI assistant that uses OpenAI's GPT-3 to generate CLI commands based on user prompts. It can also explain the generated command in plain English.

## Installation

1. Clone this repository
2. Install Rust and Cargo
3. Run `cargo build --release` in the project directory

## Usage

CLI-Pilot requires an OpenAI API key to function. Set the `openai_api_key` config variable to your API key before running CLI-Pilot.

To set a configuration key, run `cargo run set openai_api_key=<your_key>`.

Asking cli-pilot for help:

```
gpt list all files in current directory
```
or 
```
gpt fetch me current weather in Paris
```

After generating a command, the user is presented with a menu that allows them to run the command, revise the prompt, or cancel the operation.
