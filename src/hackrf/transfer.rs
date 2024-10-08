use std::{any::Any, slice};

use super::{ffi, HackRf};

pub type TransmitCallback = fn(&HackRf, &mut [u8], &dyn Any);
pub type ReceiveCallback = fn(&HackRf, &[u8], &dyn Any);

pub struct TransferContext<Callback> {
    callback: Callback,
    hackrf: HackRf,
    user_data: Box<dyn Any>,
}

impl<Callback> TransferContext<Callback> {
    pub(super) fn new(callback: Callback, hackrf: HackRf, user_data: Box<dyn Any>) -> Self {
        Self {
            callback,
            hackrf,
            user_data,
        }
    }
}

pub(super) extern "C" fn tx_callback(transfer: *mut ffi::HackrfTransfer) -> i32 {
    unsafe {
        let transfer = &mut *transfer;
        let context = &*(transfer.tx_ctx as *mut TransferContext<TransmitCallback>);

        let buffer = slice::from_raw_parts_mut(transfer.buffer, transfer.buffer_length as usize);
        (context.callback)(&context.hackrf, buffer, &*context.user_data);
    }

    0
}

pub(super) extern "C" fn rx_callback(transfer: *mut ffi::HackrfTransfer) -> i32 {
    unsafe {
        let transfer = &*transfer;
        let context = &*(transfer.rx_ctx as *mut TransferContext<ReceiveCallback>);

        let buffer = slice::from_raw_parts(transfer.buffer, transfer.buffer_length as usize);
        (context.callback)(&context.hackrf, buffer, &*context.user_data);
    }

    0
}
