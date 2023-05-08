#![no_main]
#![no_std]

use libtock::{ble::Ble, console::Console};
use libtock_runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x400}

fn main() {
    // TODO: To implement
}
