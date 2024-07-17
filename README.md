# Dairy Farming Project on Internet Computer Blockchain

This project implements a backend canister written in Rust for managing dairy farming operations on the Internet Computer (IC) blockchain. It includes functionality for adding cows, updating cow information, recording milk production, and retrieving cow data.

## Prerequisites

- [DFINITY SDK](https://sdk.dfinity.org/)
- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

## Project Setup

### 1. Install DFINITY SDK

```bash
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

### 2. Install Rust and add the WebAssembly target

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

### 3. Clone and Navigate to the Project

```bash
git clone https://github.com/YourUsername/dairy.git
cd dairy
```

### 4. Project Structure

Ensure your project has the following structure:

```
dairy/
├── Cargo.toml
├── dfx.json
└── src/
    └── dairy_backend/
           ├── src
           |    └── lib.rs
           └── dairy_backend.did
```

### 5. Update Cargo.toml

Ensure your `Cargo.toml` contains the following:

```toml
[package]
name = "dairy_backend"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.8.4"
ic-cdk = "0.7.0"
ic-cdk-macros = "0.6.0"
serde = "1.0.152"
```

### 6. Update dfx.json

Ensure your `dfx.json` contains the following:

```json
{
  "canisters": {
    "dairy_backend": {
      "candid": "src/dairy_backend/dairy_backend.did",
      "package": "dairy_backend",
      "type": "rust"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "version": 1
}
```

## Building and Deploying

1. Start the local Internet Computer network:

```bash
dfx start --background
```

2. Deploy the canister:

```bash
dfx deploy
```

## Interacting with the Canister

You can interact with the canister using `dfx canister call` commands:

1. Add a Cow:

```bash
dfx canister call dairy_backend add_cow '("Bessie", 4)'
```

2. Update a Cow:

```bash
dfx canister call dairy_backend update_cow '(1, opt "Daisy", opt 5)'
```

3. Record Milk Production:

```bash
dfx canister call dairy_backend record_milk_production '(1, 10)'
```

4. Get Cow Details:

```bash
dfx canister call dairy_backend get_cow '(1)'
```

5. Get All Cows:

```bash
dfx canister call dairy_backend get_all_cows
```

6. Update Health Status:

```bash
dfx canister call dairy_backend update_health_status '(1, "Excellent")'
```

7. Get Total Milk Production:

```bash
dfx canister call dairy_backend get_total_milk_production
```
These operations allow for complete management of cow records in the dairy farming system.

## Accessing the Candid UI

You can access the Candid UI to interact with your canister visually. After deploying, the console will display a link similar to:

```
http://127.0.0.1:8000/?canisterId=<canister-id>
```

Replace `<canister-id>` with the actual canister ID from the deployment output.

## Development Resources

- [Internet Computer Developer Documentation](https://internetcomputer.org/docs/current/developer-docs/)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [Candid Guide](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.