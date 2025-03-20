# HTLC Smart Contract

This contract implements a Hashed TimeLock Contract (HTLC) on the Soroban blockchain, enabling conditional token transfers based on a secret and time lock.

## Overview

The HTLC smart contract allows users to:

- Lock tokens in a contract using a hash-based condition.
- Provide the secret preimage (data) to claim the locked tokens before expiration.
- Cancel the contract and refund tokens if the contract has expired.

## Key Features

- **Contract Creation:** Lock tokens with a defined expiration timestamp and a hash condition.
- **Data Provision:** Submit data to verify the correct secret; tokens are then released to the designated recipient.
- **Expiration Handling:** Cancel expired contracts to refund tokens appropriately.
- **Error Handling:** Provides descriptive errors such as:
  - NotFound
  - NotExpiredYet
  - AlreadyExpired
  - InvalidSignature

## Project Structure

The repository for this contract follows a standard Soroban project layout:

.
├── Cargo.toml # Top-level workspace configuration
├── Makefile # Build, test, format, and clean tasks
├── README.md # Project overview (this file)
└── src
├── lib.rs # Main contract logic and endpoints
├── storage.rs # Persistent storage definitions and helper functions
└── test.rs # Unit tests for the contract

## Contract API

The contract exposes the following functions:

- **create(env, from, to, token, amount, expired_at, hash) -> u64**
  Creates an HTLC by moving tokens from the sender to the contract. Returns the contract ID.

- **cancel_expired(env, id) -> Result&lt;bool, Error&gt;**
  Cancels an HTLC if it has expired. Transfers tokens from the contract to the designated recipient.

- **provide_data(env, id, data) -> Result&lt;bool, Error&gt;**
  Provides the secret data. If the provided data's hash matches the stored hash and the contract is not expired, tokens are released to the recipient.

- **get_event(env, id) -> Option&lt;DataItem&gt;**
  Retrieves the stored data for a given contract.

## Development & Testing

- **Build the contract:**
  Run `make build` to compile the contract and produce the WASM file.

- **Run tests:**
  Execute `make test` to run unit tests and ensure contract functionality.

- **Formatting:**
  Use `make fmt` to format the codebase.

## Contributing

Contributions, issues, and feature requests are welcome. Please follow the standard contribution guidelines for this project.

## License

[Specify the license under which this contract is distributed.]
