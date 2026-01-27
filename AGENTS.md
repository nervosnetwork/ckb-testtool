# AGENTS.md - Guidance for AI Coding Agents

This document provides guidelines for AI agents working on the `ckb-testtool` codebase.

## Project Overview

`ckb-testtool` is a Rust library for testing CKB (Nervos Common Knowledge Base) smart contracts.
It provides a `Context` struct for deploying contracts, building transactions, and verifying them
using CKB-VM. Originally migrated from [capsule](https://github.com/nervosnetwork/capsule).

**Rust Edition:** 2024  
**Rust Toolchain:** 1.85.0 (specified in `rust-toolchain`)

## Build & Test Commands

| Command | Description |
|---------|-------------|
| `cargo build` | Build the library |
| `cargo build --features="native-simulator"` | Build with native-simulator |
| `cargo fmt --all -- --check` | Check formatting |
| `cargo clippy --all` | Run clippy linting |
| `cargo test --all` | Run all tests |
| `cargo test test_name` | Run specific test by name |
| `cargo test test_name -- --nocapture` | Run test with output |
| `cargo test --features="native-simulator" --all` | Run with native-simulator |
| `cargo test --test normal_cases` | Run tests in specific file |
| `cd tests/test-contracts && make build` | Build test contracts |
| `rustup target add riscv64imac-unknown-none-elf` | Add RISC-V target |

## Project Structure

```
ckb-testtool/
├── Cargo.toml              # Main crate manifest
├── rust-toolchain          # Rust version: 1.85.0
├── rustfmt.toml            # Formatting configuration
├── src/
│   ├── lib.rs              # Crate entry with re-exports and docs
│   ├── context.rs          # Main Context struct for testing
│   ├── builtin.rs          # Built-in contracts (ALWAYS_SUCCESS)
│   └── tx_verifier.rs      # Transaction verification (private)
└── tests/
    ├── normal_cases.rs     # Standard test cases
    ├── native_simulator_cases.rs  # Native simulator tests
    └── test-contracts/     # Test contracts workspace
```

## Feature Flags

- `native-simulator`: Enables native simulation testing with `libloading` and `serde_json`

## Code Style Guidelines

### Formatting (rustfmt.toml)

```toml
max_width = 100
tab_spaces = 4
reorder_imports = true
reorder_modules = true
use_try_shorthand = true
```

Always run `cargo fmt` before committing.

### Import Organization

Order imports as follows:
1. Crate-level imports (`crate::`)
2. External crate imports
3. Standard library imports (`std::`)

Use nested imports for multiple items from the same module:

```rust
use crate::tx_verifier::OutputsDataVerifier;
use ckb_types::{
    bytes::Bytes,
    core::{Capacity, Cycle, TransactionView},
    packed::{Byte32, CellDep, CellOutput, OutPoint, Script},
    prelude::*,
};
use std::collections::HashMap;
```

### Naming Conventions

| Element     | Convention          | Example                            |
|-------------|---------------------|-----------------------------------|
| Functions   | snake_case          | `deploy_cell`, `verify_tx`        |
| Types       | PascalCase          | `Context`, `Message`              |
| Variables   | snake_case          | `out_point`, `data_hash`          |
| Constants   | SCREAMING_SNAKE_CASE| `MAX_CYCLES`, `SIGNATURE_SIZE`    |
| Modules     | snake_case          | `tx_verifier`, `context`          |

### Type Annotations

- Always specify return types for public functions
- Use explicit types when not obvious from context
- Use lifetime annotations when required

```rust
pub fn random_hash() -> Byte32 { ... }
pub fn verify_tx(&self, tx: &TransactionView, max_cycles: u64) -> Result<Cycle, CKBError> { ... }
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use `Option<T>` for optional returns
- Prefer `?` operator for error propagation
- Use `.expect("descriptive message")` for programmer errors
- Use `.unwrap_or_else()` for complex error handling

```rust
pub fn verify_tx(&self, tx: &TransactionView, max_cycles: u64) -> Result<Cycle, CKBError> {
    self.verify_tx_consensus(tx)?;
    // ...
}

let path = self.get_contract_path(filename).expect("get contract path");
```

### Documentation

- Use `//!` for module-level documentation
- Use `///` for public API documentation
- Include `# Example` sections with `rust,no_run` code blocks

### Test Structure

- Use `#[test]` attribute for test functions
- Define constants like `MAX_CYCLES` at the top
- Follow the pattern: setup context -> deploy contracts -> build tx -> verify

```rust
const MAX_CYCLES: u64 = 500_0000;

#[test]
fn test_example() {
    let mut context = Context::default();
    let out_point = context.deploy_cell(contract_bin);
    // ... build transaction ...
    context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
}
```

### Numeric Literals

Use underscores for readability: `500_0000`, `500_000_000_000`

## Key APIs

- `Context::default()` - Create new context
- `context.deploy_cell(data)` - Deploy contract, returns `OutPoint`
- `context.deploy_cell_by_name(name)` - Deploy contract from search path
- `context.build_script(&out_point, args)` - Build script from deployed contract
- `context.create_cell(output, data)` - Create input cell
- `context.complete_tx(tx)` - Add cell deps automatically
- `context.verify_tx(&tx, max_cycles)` - Verify transaction

## CI Pipeline

The CI runs on every push and PR:
1. Format check: `cargo fmt --all -- --check`
2. Clippy: `cargo clippy --all`
3. Build: `cargo build`
4. Tests: `cargo test --all`
5. Native simulator tests: `cargo test --features="native-simulator" --all`

Ensure all checks pass before submitting PRs.
