#![no_std]
#![no_main]
//! Has the User LED on PA5 turn on when the User Button on PC13 is pressed, and turn off when it is released.
//!
//! This is the Rust-Only implementation.
//!

use cortex_m_rt::entry;
use panic_halt as _;

use constants::memory_access::{reg32_read, reg32_write};
use constants::stm32l476xx_constants::*;

const LED_PIN: u32 = 5;
const USER_PIN: u32 = 13;

#[entry]
fn main() -> ! {
    enable_gpio_ports();
    configure_gpio_a();
    configure_gpio_c();

    let mut idr: u32;
    let mut odr: u32;

    loop {
        idr = !reg32_read(GPIOC_BASE + GPIO_IDR);
        idr &= 0b1 << USER_PIN;
        idr >>= USER_PIN - LED_PIN;

        odr = reg32_read(GPIOA_BASE + GPIO_ODR);
        odr &= !(0b1 << LED_PIN);
        odr |= idr;
        reg32_write(GPIOA_BASE + GPIO_ODR, odr);
    }
}

fn enable_gpio_ports() {
    let rcc_enable = RCC_BASE + RCC_AHB2ENR;

    let mut enable_contents = reg32_read(rcc_enable);

    enable_contents |= RCC_AHB2ENR_GPIOAEN | RCC_AHB2ENR_GPIOCEN;

    reg32_write(rcc_enable, enable_contents);
}

fn configure_gpio_a() {
    {
        let mut moder = reg32_read(GPIOA_BASE + GPIO_MODER);

        moder &= !(0b11 << (2 * LED_PIN));
        moder |= 0b01 << (2 * LED_PIN);

        reg32_write(GPIOA_BASE + GPIO_MODER, moder);
    }
    {
        let mut otyper = reg32_read(GPIOA_BASE + GPIO_OTYPER);

        otyper &= !(0b1 << LED_PIN);

        reg32_write(GPIOA_BASE + GPIO_OTYPER, otyper);
    }
}

fn configure_gpio_c() {
    {
        let mut moder = reg32_read(GPIOC_BASE + GPIO_MODER);

        moder &= !(0b11 << (2 * USER_PIN));

        reg32_write(GPIOC_BASE + GPIO_MODER, moder);
    }
    {
        let mut pupdr = reg32_read(GPIOC_BASE + GPIO_PUPDR);

        pupdr &= !(0b11 << (2 * USER_PIN));

        reg32_write(GPIOC_BASE + GPIO_PUPDR, pupdr);
    }
}
