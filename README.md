# CLI-Pilot
CLI-Pilot is a Rust-based personal CLI assistant that uses OpenAI's GPT to generate CLI commands based on user prompts. It can also explain the generated command in plain English. Project was created for learning Rust purposes.

<p align="center">
  <img src="https://raw.githubusercontent.com/Kukiezi/cli-pilot/main/assets/weather.gif" alt="animated" />
</p>

## Installation

1. Clone this repository
2. Install Rust and Cargo
3. Run `cargo build --release` in the project directory

## Setup API Key

CLI-Pilot requires an OpenAI API key to function. Set the `openai_api_key` config variable to your API key before running CLI-Pilot.

To set a configuration key, run `gpt set openai_api_key=<your_key>`.

## Usage

To generate a command via cli-pilot you have to use this command:
```
gpt <PROMPT>
```

<p align="start">
  <img src="https://raw.githubusercontent.com/Kukiezi/cli-pilot/main/assets/listfiels.gif" alt="animated" />
</p>

```
gpt list all files in current directory
```

After generating a command, the user is presented with a menu that allows them to run the command, revise the prompt, or cancel the operation.

And that is it!

## Contributing

CLI-Pilot was made mostly for fun and for learning Rust. However, any contributions to improve the code, bring new features, write tests, or fix bugs are more than welcome! 

Thank you for contributing to CLI-Pilot!
