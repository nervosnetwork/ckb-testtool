//! ckb-testtool
//!
//! This module provides testing context for CKB contracts.
//!
//! To setup a contract verification context, you may need to import ckb modules to build the transaction structure or
//! calculate the hash result.
//!
//! `ckb-testtool` crate provides re-exports of ckb modules.
//!
//! # Example
//!
//! ```rust,no_run
//! use ckb_testtool::context::Context;
//! use ckb_testtool::ckb_types::{
//!     bytes::Bytes,
//!     core::TransactionBuilder,
//!     packed::*,
//!     prelude::*,
//! };
//! use std::fs;
//!
//! // Max cycles of verification.
//! const MAX_CYCLES: u64 = 10_000_000;
//!
//! #[test]
//! fn test_basic() {
//!     // Init testing context.
//!     let mut context = Context::default();
//!     let contract_bin: Bytes = fs::read("my_contract").unwrap().into();
//!
//!     // Deploy contract.
//!     let out_point = context.deploy_cell(contract_bin);
//!
//!     // Prepare scripts and cell dep.
//!     let lock_script = context
//!         .build_script(&out_point, Default::default())
//!         .expect("script");
//!
//!     // Prepare input cell.
//!     let input_out_point = context.create_cell(
//!         CellOutput::new_builder()
//!             .capacity(1000u64.pack())
//!             .lock(lock_script.clone())
//!             .build(),
//!         Bytes::new(),
//!     );
//!     let input = CellInput::new_builder()
//!         .previous_output(input_out_point)
//!         .build();
//!
//!     // Outputs.
//!     let outputs = vec![
//!         CellOutput::new_builder()
//!             .capacity(500u64.pack())
//!             .lock(lock_script.clone())
//!             .build(),
//!         CellOutput::new_builder()
//!             .capacity(500u64.pack())
//!             .lock(lock_script)
//!             .build(),
//!     ];
//!
//!     let outputs_data = vec![Bytes::new(); 2];
//!
//!     // Build transaction.
//!     let tx = TransactionBuilder::default()
//!         .input(input)
//!         .outputs(outputs)
//!         .outputs_data(outputs_data.pack())
//!         .build();
//!
//!     let tx = context.complete_tx(tx);
//!
//!     // Run.
//!     let cycles = context
//!         .verify_tx(&tx, MAX_CYCLES)
//!         .expect("pass verification");
//!     println!("consume cycles: {}", cycles);
//! }
//! ```
//!
//! The `ckb-testtool` also supports native simulation. To use this mode, you need to enable the `native-simulator`
//! feature. Next, you should create a new native simulation project. In the new project, you only need to add one
//! line in main.rs:
//!
//! ```text
//! ckb_std::entry_simulator!(script::program_entry);
//! ```
//!
//! Recompile it. ckb-testtool will automatically locate it in the contract search path. You can refer to path
//! `tests/test-contracts/native-simulators/simple-spawn-sim` for relevant examples.

pub mod builtin;
pub mod context;
mod tx_verifier;

// re-exports
pub use ckb_chain_spec;
pub use ckb_crypto;
pub use ckb_error;
pub use ckb_hash;
pub use ckb_jsonrpc_types;
pub use ckb_script;
pub use ckb_traits;
pub use ckb_types;
pub use ckb_types::bytes;
pub use ckb_verification;
