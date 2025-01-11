# Turbin3 Prerequisite Course Program

ps: more trust issues regarding PDFs 


This repository contains a Rust implementation to interact with the Turbin3 Prerequisite Course Program deployed on the Solana Devnet. This project consumes the program's IDL (Interface Definition Language) to invoke the `complete` function, proving successful completion of the Turbin3 pre-requisite coursework.

---

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Setup Instructions](#setup-instructions)
- [Usage](#usage)
- [Folder Structure](#folder-structure)
- [PDA (Program Derived Address)](#pda-program-derived-address)
- [Solana Explorer](#solana-explorer)
- [License](#license)

---

## Overview
The Turbin3 Prerequisite Course Program is published on the Solana Devnet. It allows users to:
- Submit their GitHub username to prove completion of the course.
- Interact with the blockchain using a custom Program Derived Address (PDA).

The program's IDL provides the schema for the instruction and accounts required to invoke the `complete` function.

---

## Features
- Connect to the Solana Devnet using an RPC client.
- Generate the PDA required for the `prereq` account.
- Submit transactions to the Solana blockchain.
- View transaction details via Solana Explorer.

---

## Setup Instructions

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).
- Install [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools).
- Clone this repository:
  ```bash
  git clone <repository-url>
  cd <repository-folder>
  ```
- Install required dependencies by adding them to `Cargo.toml`:
  ```toml
  [dependencies]
  borsh = "0.10.3"
  solana-idlgen = { git = "https://github.com/deanmlittle/solana-idlgen.git" }
  solana-sdk = "1.14.14"
  ```

### Folder Structure Setup
1. Create a folder for program modules:
   ```bash
   mkdir src/programs
   touch src/programs/mod.rs
   touch src/programs/Turbin3_prereq.rs
   ```

2. Add the following to `src/programs/mod.rs`:
   ```rust
   pub mod Turbin3_prereq;
   ```

3. Define the IDL in `src/programs/Turbin3_prereq.rs` using the `idlgen!` macro:
   ```rust
   use solana_idlgen::idlgen;

   idlgen!({
       "version": "0.1.0",
       "name": "Turbin3_prereq",
       "instructions": [{
           "name": "complete",
           "args": [{ "name": "github", "type": "bytes" }],
           "accounts": [
               { "name": "signer", "isSigner": true, "isWritable": false },
               { "name": "prereq", "isSigner": false, "isWritable": true },
               { "name": "systemProgram", "isSigner": false, "isWritable": false }
           ]
       }],
       "metadata": {
           "address": "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1"
       }
   });
   ```

4. Add the following to `lib.rs`:
   ```rust
   mod programs;
   ```

---

## Usage

### Step 1: Set Up Wallet
- Create or import a wallet using the Solana CLI:
  ```bash
  solana-keygen new --outfile Turbin3-wallet.json
  ```
- Store the wallet file securely. This wallet will be used as the signer for transactions.

### Step 2: Submit a Transaction
Use the `complete_prereq_course` function in `lib.rs` to interact with the Turbin3 program:

```rust
pub fn complete_prereq_course() {
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());

    // Define accounts (signer and PDA)
    let signer = read_keypair_file("Turbin3-wallet.json")
        .expect("Couldn't find wallet file");
    let prereq = Turbin3PrereqProgram::derive_program_address(&[
        b"prereq",
        signer.pubkey().to_bytes().as_ref()
    ]);

    // Define instruction data (GitHub username)
    let args = CompleteArgs {
        github: b"your_github_username".to_vec(),
    };

    // Fetch blockhash and create the transaction
    let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");
    let instruction = Turbin3PrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
    );

    // Send transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!("Success! Transaction: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}
```

### Step 3: Run the Function
Call the `complete_prereq_course()` function in your test module or main function to execute the transaction.

---

## Folder Structure
```
project-root/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── programs/
│   │   ├── mod.rs
│   │   └── Turbin3_prereq.rs
│
└── Turbin3-wallet.json
```

---

## PDA (Program Derived Address)
A Program Derived Address (PDA) ensures unique, deterministic account addresses for your program. The PDA for this project is derived using:
1. The string seed: `"prereq"`
2. The public key of the transaction signer.

```rust
let prereq = Turbin3PrereqProgram::derive_program_address(&[
    b"prereq",
    signer.pubkey().to_bytes().as_ref()
]);
```

---

## Solana Explorer
You can verify your transactions on Solana Explorer by visiting:
```
https://explorer.solana.com/tx/<transaction-signature>/?cluster=devnet
```
Replace `<transaction-signature>` with the actual transaction signature from the output.

---

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

