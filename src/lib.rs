// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;


use alloc::vec::Vec;
use stylus_sdk::prelude::entrypoint;

#[entrypoint]
fn user_main(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
    Ok(input)
}