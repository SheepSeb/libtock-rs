use crate::{DriverInfo, DriverShareRef};
use libtock_platform::{CommandReturn, ErrorCode};
use std::cell::Cell;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UsbData {
    pub fired: bool,
    pub rcode: usize,
}

pub struct Usb {
    busy: Cell<bool>,
    upcall_on_command: Cell<Option<UsbData>>,
    share_ref: DriverShareRef,
}

impl Usb {
    pub fn new() -> std::rc::Rc<Usb> {
        std::rc::Rc::new(Usb {
            busy: Cell::new(false),
            upcall_on_command: Cell::new(None),
            share_ref: Default::default(),
        })
    }

    pub fn is_busy(&self) -> bool {
        self.busy.get()
    }

    pub fn set_value(&self, value: UsbData) {
        if self.busy.get() {
            self.share_ref
                .schedule_upcall(0, (value.fired as u32, value.rcode as u32, 0))
                .expect("Unable to schedule upcall");
            self.busy.set(false);
        }
    }

    pub fn set_value_sync(&self, value: UsbData) {
        self.upcall_on_command.set(Some(value));
    }
}
impl crate::fake::SyscallDriver for Usb {
    fn info(&self) -> DriverInfo {
        DriverInfo::new(DRIVER_NUM).upcall_count(1)
    }

    fn register(&self, share_ref: DriverShareRef) {
        self.share_ref.replace(share_ref);
    }

    fn command(&self, command_id: u32, _argument0: u32, _argument1: u32) -> CommandReturn {
        match command_id {
            EXISTS => crate::command_return::success(),
            ENABLE => {
                if self.busy.get() {
                    return crate::command_return::failure(ErrorCode::Busy);
                }
                self.busy.set(true);
                if let Some(val) = self.upcall_on_command.take() {
                    self.set_value(val);
                }
                crate::command_return::success()
            }
            _ => crate::command_return::failure(ErrorCode::NoSupport),
        }
    }
}

const DRIVER_NUM: u32 = 0x20005;

// Command IDs
const EXISTS: u32 = 0;
const ENABLE: u32 = 1;
