use crate::fake::{self, SyscallDriver};
use fake::usb::*;
use libtock_platform::{share, DefaultConfig, YieldNoWaitReturn};

#[test]
fn command() {
    let usb = Usb::new();

    assert(usb.command(EXISTS, 0, 0).is_success());
}
