// SPDX-License-Identifier: GPL-2.0
#![no_std]
#![feature(allocator_api)]

pub const REALTEK_VENDOR_ID: u16 = 0x10ec;
pub const RTL8169_DEVICE_ID: u16 = 0x8169;

pub enum RtlHwState {
    Uninitialized,
    ResetPending,
    DmaRingReady,
    Active,
}

pub struct RtlDmaRing {
    pub tx_desc_count: usize,
    pub rx_desc_count: usize,
}
