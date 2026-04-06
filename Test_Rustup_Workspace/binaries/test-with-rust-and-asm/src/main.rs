#![no_std]
#![no_main]
//! Has the User LED on PA5 turn on when the User Button on PC13 is pressed, and turn off when it is released.
//!
//! This implementation uses Rust to call the included assembly from test.s
//!

use cortex_m_rt::entry;
use panic_halt as _;

core::arch::global_asm!(include_str!("test.s"));

unsafe extern "C" {
    fn enable_gpio_ports();
    fn configure_gpio_a();
    fn configure_gpio_c();
    fn main_loop() -> !;
}

#[entry]
fn main() -> ! {
    unsafe {
        enable_gpio_ports();
        configure_gpio_a();
        configure_gpio_c();
        main_loop();
    }
}
