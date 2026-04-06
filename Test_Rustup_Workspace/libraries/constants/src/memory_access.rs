use core::ptr::{read_volatile, write_volatile};

/// Ensure that the address is aligned to 8 bytes
#[inline]
pub fn reg64_read(addr: u32) -> u64 {
    if !addr.is_multiple_of(8) {
        panic!("Misaligned read!")
    }
    unsafe { read_volatile(addr as *const u64) }
}

/// Ensure that the address is aligned to 8 bytes
#[inline]
pub fn reg64_write(addr: u32, value: u64) {
    if !addr.is_multiple_of(8) {
        panic!("Misaligned write!")
    }
    unsafe { write_volatile(addr as *mut u64, value) }
}

/// Ensure that the address is aligned to 4 bytes
#[inline]
pub fn reg32_read(addr: u32) -> u32 {
    if !addr.is_multiple_of(4) {
        panic!("Misaligned read!")
    }
    unsafe { read_volatile(addr as *const u32) }
}

/// Ensure that the address is aligned to 4 bytes
#[inline]
pub fn reg32_write(addr: u32, value: u32) {
    if !addr.is_multiple_of(4) {
        panic!("Misaligned write!")
    }
    unsafe { write_volatile(addr as *mut u32, value) }
}

/// Ensure that the address is aligned to 2 bytes
#[inline]
pub fn reg16_read(addr: u32) -> u16 {
    if !addr.is_multiple_of(2) {
        panic!("Misaligned read!")
    }
    unsafe { read_volatile(addr as *const u16) }
}

/// Ensure that the address is aligned to 2 bytes
#[inline]
pub fn reg16_write(addr: u32, value: u16) {
    if !addr.is_multiple_of(2) {
        panic!("Misaligned write!")
    }
    unsafe { write_volatile(addr as *mut u16, value) }
}

#[inline]
pub fn reg8_read(addr: u32) -> u8 {
    unsafe { read_volatile(addr as *const u8) }
}

#[inline]
pub fn reg8_write(addr: u32, value: u8) {
    unsafe { write_volatile(addr as *mut u8, value) }
}
