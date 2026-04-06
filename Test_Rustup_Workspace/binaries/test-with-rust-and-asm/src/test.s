.equ PERIPH_BASE, 0x40000000
.equ AHB1PERIPH_BASE, (PERIPH_BASE + 0x00020000)
.equ AHB2PERIPH_BASE, (PERIPH_BASE + 0x08000000)
.equ RCC_BASE, (AHB1PERIPH_BASE + 0x1000)
.equ GPIOA_BASE, AHB2PERIPH_BASE
.equ GPIOC_BASE, (AHB2PERIPH_BASE + 0x0800)
.equ RCC_AHB2ENR, 0x4C
.equ RCC_AHB2ENR_GPIOAEN, 0x00000001
.equ RCC_AHB2ENR_GPIOCEN, 0x00000004
.equ GPIO_MODER, 0x00
.equ GPIO_OTYPER, 0x04
.equ GPIO_PUPDR, 0x0C
.equ GPIO_IDR, 0x10
.equ GPIO_ODR, 0x14

.equ LED_PIN, 5
.equ USER_PIN, 13

.global enable_gpio_ports
.global configure_gpio_a
.global configure_gpio_c
.global main_loop

enable_gpio_ports:
    LDR r0, =RCC_BASE

    LDR r1, [r0, #RCC_AHB2ENR]
    ORR r1, r1, #RCC_AHB2ENR_GPIOAEN
    ORR r1, r1, #RCC_AHB2ENR_GPIOCEN
    STR r1, [r0, #RCC_AHB2ENR]

    BX lr

configure_gpio_a:
    LDR r0, =GPIOA_BASE

    LDR r1, [r0, #GPIO_MODER]
    BIC r1, r1, #(0b11 << (2 * LED_PIN))
    ORR r1, r1, #(0b01 << (2 * LED_PIN))
    STR r1, [r0, #GPIO_MODER]

    LDR r1, [r0, #GPIO_OTYPER]
    BIC r1, r1, #(0b1 << LED_PIN)
    STR r1, [r0, #GPIO_OTYPER]

    BX lr

configure_gpio_c:
    LDR r0, =GPIOC_BASE
    LDR r1, [r0, #GPIO_MODER]
    BIC r1, r1, #(0b11 << (2 * USER_PIN))
    STR r1, [r0, #GPIO_MODER]

    LDR r1, [r0, #GPIO_PUPDR]
    BIC r1, r1, #(0b11 << (2 * USER_PIN))
    STR r1, [r0, #GPIO_PUPDR]

    BX lr

main_loop:
    LDR r0, =GPIOA_BASE
    LDR r1, =GPIOC_BASE
    
loop:
        LDR r2, [r1, #GPIO_IDR]
        MVN r2, r2
        AND r2, r2, #(0b1 << USER_PIN)
        LSR r2, r2, #(USER_PIN - LED_PIN)

        LDR r3, [r0, #GPIO_ODR]
        BIC r3, r3, #(0b1 << LED_PIN)
        ORR r3, r3, r2
        STR r3, [r0, #GPIO_ODR]

        B loop
