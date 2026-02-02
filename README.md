# Rust Matrix Bot

This is a simple bot written in Rust that integrates with Matrix and Firefly III. The bot listens to Matrix messages and creates transactions in Firefly III based on the content of those messages. It is primarily used for logging cash transactions.

## Features
- Listens to Matrix messages.
- Creates transactions in Firefly III.
- Designed for logging cash transactions.

## Requirements
- Rust 
- Firefly III instance
- Matrix account

## Setup
1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd rust_matrix_bot
   ```
2. Install dependencies:
   ```bash
   cargo build
   ```
3. Configure the bot by editing the `config.toml` file with your Matrix and Firefly III credentials.

## Usage
Run the bot using the following command:
```bash
cargo run
```

## License
This project is licensed under the MIT License.