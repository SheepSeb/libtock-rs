#![no_main]
#![no_std]

use libtock::ble::{self, BLE};
use libtock_runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x400}

fn main() {
    if BLE::ble_start_advertising().is_err() {
        panic!("Error starting advertising");
    }
}
