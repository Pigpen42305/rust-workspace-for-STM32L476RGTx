use core::arch::asm;

#[inline]
pub fn bic(op1: u32, op2: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "bic {0}, {1}, {2}",
            out(reg) result,
            in(reg) op1,
            in(reg) op2,
            options(nomem, nostack, preserves_flags, pure),
        );
    }
    result
}

#[inline]
pub fn bfi<const LSB: u8, const WIDTH: u8>(op1: u32, op2: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "bfi {0}, {1}, #{2}, #{3}",
            inlateout(reg) op1 => result,
            in(reg) op2,
            const LSB,
            const WIDTH,
            options(nomem, nostack, preserves_flags, pure),
        );
    }
    result
}
