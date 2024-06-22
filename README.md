# Dairy Farming Project on Internet Computer Blockchain

This project is a backend canister written in Rust for managing dairy farming operations, including adding cows, recording milk production, and retrieving cow data. The canister is deployed on the Internet Computer (IC) blockchain.

## Prerequisites

- [DFINITY SDK](https://sdk.dfinity.org/)
- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

## Project Setup

### 1. Install DFINITY SDK

```bash
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
rustup target add wasm32-unknown-unknown
```

### 3. Clone the Project

```bash
Copy code
git clone https://github.com/H3nryK/dairy.git
cd dairy
```

### 4. Create the Project Structure

```bash
Copy code
dfx new dairy_farming
cd dairy_farming
```

### 5. Update Cargo.toml

Edit Cargo.toml to include the necessary dependencies:

```bash
[package]
name = "dairy_backend"
version = "0.1.0"
edition = "2018"

[dependencies]
ic-cdk = "0.14.0"
ic-cdk-macros = "0.14.0"
candid = "0.10.9"
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"

[lib]
crate-type = ["cdylib"]
```

### 6. Implement the Canister Logic

Edit the `lib.rs` file.

### 7. Generate the Candid Interface

Build the project and generate the Candid interface file:

```bash
cargo build --target wasm32-unknown-unknown --release
cargo test -- --nocapture
```

### 9. Deploy the Canister

Deploy your canister to the local Internet Computer instance:

```bash
dfx deploy
```

## Interacting with the Canister

1. Add a Cow

```bash
dfx canister call dairy_backend add_cow '(1, "Bessie", 4)'
```

2. Record Milk Production

```bash
dfx canister call dairy_backend record_milk_production '(1, 10)'
```

3. Get Cow Details

```bash
dfx canister call dairy_backend get_cow '(1)'
```

## Accessing the Candid UI

You can access the Candid UI to interact with your canister at:

```bash
http://localhost:8000/?canisterId=<your-canister-id>
```

Replace <your-canister-id> with the actual canister ID obtained from the deployment process.
