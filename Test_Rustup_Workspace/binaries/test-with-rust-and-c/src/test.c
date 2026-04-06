#include "stm32l476xx.h"
#include "crust.h"

#define LED_PIN 5
#define USER_PIN 13

void enable_gpio_ports(void) {
    u32 rcc_ahb2enr = RCC->AHB2ENR;
    rcc_ahb2enr |= RCC_AHB2ENR_GPIOAEN;
    rcc_ahb2enr |= RCC_AHB2ENR_GPIOCEN;
    RCC->AHB2ENR = rcc_ahb2enr;
}

void configure_gpio_a(void) {
    u32 moder = GPIOA->MODER;
    moder &= ~(0b11UL << (2 * LED_PIN));
    moder |= (0b01UL << (2 * LED_PIN));
    GPIOA->MODER = moder;

    GPIOA->OTYPER = ~(0b1UL << LED_PIN);
}

void configure_gpio_c(void) {
    GPIOC->MODER &= ~(0b11UL << (2 * USER_PIN));

    GPIOC->PUPDR &= ~(0b11UL << (2 * USER_PIN));
}

void main_loop(void) {
    u32 idr;
    u32 odr;

    while (1) {
        idr = ~GPIOC->IDR;
        idr &= 0b1UL << USER_PIN;
        idr >>= USER_PIN - LED_PIN;

        odr = GPIOA->ODR;
        odr &= ~(0b1UL << LED_PIN);
        odr |= idr;

        GPIOA->ODR = odr;
    }
}