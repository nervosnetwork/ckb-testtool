#![cfg_attr(not(any(feature = "native-simulator", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "native-simulator", test))]
extern crate alloc;

#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::default_alloc!();

use alloc::vec::Vec;
use ckb_std::{ckb_constants::Source, ckb_types::core::ScriptHashType};
use core::ffi::CStr;

fn parent_entry() -> i8 {
    let args = ["test"];
    let args_buf: Vec<Vec<u8>> = args.iter().map(|f| [f.as_bytes(), &[0]].concat()).collect();
    let c_args: Vec<&CStr> = args_buf
        .iter()
        .map(|f| unsafe { CStr::from_bytes_with_nul_unchecked(f) })
        .collect();

    let inherited_fds = [0];

    let pid = ckb_std::high_level::spawn_cell(
        &ckb_std::high_level::load_cell_lock(0, Source::GroupInput)
            .unwrap()
            .code_hash()
            .raw_data(),
        ScriptHashType::Data2,
        &c_args,
        &inherited_fds,
    )
    .unwrap();

    ckb_std::debug!("child pid: {}", pid);

    0
}

fn child_entry() -> i8 {
    ckb_std::debug!("spawn child stared");
    0
}

pub fn program_entry() -> i8 {
    ckb_std::debug!("This is a sample contract!");

    let argv = ckb_std::env::argv();
    if argv.is_empty() {
        parent_entry()
    } else {
        child_entry()
    }
}
