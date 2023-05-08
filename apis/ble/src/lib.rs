#![no_std]

use libtock_platform::{ErrorCode, Syscalls};

pub struct Ble<S: Syscalls>(S);

impl<S: Syscalls> Ble<S> {
    pub fn ble_stop_advertising() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, BLE_ADV_STOP_CMD, 1, 0).to_result()
    }
    pub fn ble_set_tx_power(power_level: TxPower) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, BLE_CFG_TX_POWER_CMD, power_level as u32, 0).to_result()
    }
    pub fn ble_stop_passive_scan() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, BLE_ADV_STOP_CMD, 1, 0).to_result()
    }
}

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x30000;

// Command IDs

const BLE_ADV_START_CMD: u32 = 0;
const BLE_ADV_STOP_CMD: u32 = 1;
const BLE_CFG_TX_POWER_CMD: u32 = 2;
const BLE_SCAN_CMD: u32 = 5;

const ADV_IND: u32 = 0x00;
const ADV_DIRECT_IND: u32 = 0x01;
const ADV_NONCONN_IND: u32 = 0x02;
const ADV_SCAN_IND: u32 = 0x06;

#[repr(u32)]
pub enum TxPower {
    POSITIVE_10_DBM = 10,
    POSITIVE_9_DBM = 9,
    POSITIVE_8_DBM = 8,
    POSITIVE_7_DBM = 7,
    POSITIVE_6_DBM = 6,
    POSITIVE_5_DBM = 5,
    POSITIVE_4_DBM = 4,
    POSITIVE_3_DBM = 3,
    POSITIVE_2_DBM = 2,
    POSITIVE_1_DBM = 1,
    ZERO_DBM = 0,
    NEGATIVE_1_DBM = 0xff,
    NEGATIVE_2_DBM = 0xfe,
    NEGATIVE_3_DBM = 0xfd,
    NEGATIVE_4_DBM = 0xfc,
    NEGATIVE_5_DBM = 0xfb,
    NEGATIVE_6_DBM = 0xfa,
    NEGATIVE_7_DBM = 0xf9,
    NEGATIVE_8_DBM = 0xf8,
    NEGATIVE_9_DBM = 0xf7,
    NEGATIVE_10_DBM = 0xf6,
    NEGATIVE_11_DBM = 0xf5,
    NEGATIVE_12_DBM = 0xf4,
    NEGATIVE_13_DBM = 0xf3,
    NEGATIVE_14_DBM = 0xf2,
    NEGATIVE_15_DBM = 0xf1,
    NEGATIVE_16_DBM = 0xf0,
    NEGATIVE_17_DBM = 0xef,
    NEGATIVE_18_DBM = 0xee,
    NEGATIVE_19_DBM = 0xed,
    NEGATIVE_20_DBM = 0xec,
}
