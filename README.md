# Cryptomus Rust Example

This is a Rust example project for integrating with the Cryptomus API to create payments.

[Click here to go to Cryptomus website (make sure you have a registered business on there)](cryptomus.com)

## Prerequisites

Before running the project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/): Install Rust using [rustup](https://rustup.rs/).
- [Cargo](https://doc.rust-lang.org/cargo/): Cargo is installed alongside Rust.

## Setup

### 1. Clone the Repository

First, clone the repository to your local machine:

```bash
git clone https://github.com/nachodeluka/cryptomus-rust-example.git
cd cryptomus-rust-example
```

### 2. Setup the Enviroment (.env)

Rename .env.example to .env and edit your Cryptomus business data.

```
CRYPTOMUS_MERCHANT_ID=your_merchant_id
CRYPTOMUS_API_KEY=your_api_key
CRYPTOMUS_RETURNING_URL=https://your_return_url.com
CRYPTOMUS_CALLBACK_URL=https://your_callback_url.com
```

### 3. Build the project

```
cargo build
```

### 4. Run the project

```
cargo run
```


## Extra -> Used dependencies

* **reqwest**: For making HTTP requests.
* **dotenv**: For loading environment variables.
* **serde_json**: For JSON serialization and deserialization.
* **base64**: For Base64 encoding.
* **md5**: For generating MD5 hashes.

Feel free to open a new issue or PR. Thank you.
