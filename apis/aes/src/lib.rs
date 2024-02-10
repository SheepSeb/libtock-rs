#![no_std]

use core::cell::Cell;
use libtock_platform::{
    share, subscribe::OneId, DefaultConfig, ErrorCode, Subscribe, Syscalls, Upcall,
};

pub struct Aes<S: Syscalls>(S);

impl<S: Syscalls> Aes<S> {
    pub fn exists() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, EXISTS, 0, 0)
            .to_result::<u32, ErrorCode>()
            .and(Ok(()))
    }

    pub fn aes_ccm_set_confidential(value: bool) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, TOCK_AES_CCM_CONF, value as u32, 0).to_result()
    }

    pub fn aes_cmm_set_mic_len(value: u32) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, TOCK_AES_CCM_MIC_LEN, value, 0).to_result()
    }

    pub fn aes_ccm_set_m_off(value: u32) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, TOCK_AES_CCM_MOFF, value, 0).to_result()
    }

    pub fn aes_check_status() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, TOCK_AES_CHECK_PRESENT, 0, 0).to_result()
    }

    pub fn aes_set_algorithm(operation: u8, encrypting: bool) -> Result<(), ErrorCode> {
        S::command(
            DRIVER_NUM,
            TOCK_AES_SET_ALGO,
            operation as u32,
            encrypting as u32,
        )
        .to_result()
    }

    pub fn aes_setup() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, TOCK_AES_SETUP, 0, 0).to_result()
    }

    pub fn aes_crypt() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, TOCK_AES_CRYPT, 0, 0).to_result()
    }

    pub fn aes_finish() -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, TOCK_AES_FINISH, 0, 0).to_result()
    }
}

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x40006;

// Command IDs

const TOCK_AES_CB: u32 = 0;

const TOCK_AES_KEY_BUF: u32 = 0;
const TOCK_AES_IV_BUF: u32 = 1;
const TOCK_AES_SOURCE_BUF: u32 = 2;
const TOCK_AES_DEST_BUF: u32 = 0;

const TOCK_AES_CHECK_PRESENT: u32 = 0;
const TOCK_AES_SET_ALGO: u32 = 1;
const TOCK_AES_SETUP: u32 = 2;
const TOCK_AES_CRYPT: u32 = 3;
const TOCK_AES_FINISH: u32 = 4;

const TOCK_AES_CCM_AOFF: u32 = 5;
const TOCK_AES_CCM_MOFF: u32 = 6;
const TOCK_AES_CCM_MIC_LEN: u32 = 7;
const TOCK_AES_CCM_CONF: u32 = 8;
