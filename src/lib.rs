#![no_main]
extern crate alloc;

use alloc::vec::Vec;
use stylus_sdk::prelude::*;
use stylus_sdk::stylus_proc::entrypoint;

#[entrypoint]
fn user_main(input: Vec<u8>, _vm: stylus_sdk::host::VM) -> Result<Vec<u8>, Vec<u8>> {
    Ok(input)
}
