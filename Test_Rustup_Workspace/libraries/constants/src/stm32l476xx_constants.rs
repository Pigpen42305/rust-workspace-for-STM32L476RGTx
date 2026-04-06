//******************** (C) Wala Saadeh ******************************************************************
// @file    stm32l476xx_constant.s
// @author  Wala Saadeh @ WWU
// @version V1.0.0
// @date    Jan. 2025
//******************************************************************************************************

// Modified

//Peripheral base address
pub const PERIPH_BASE: u32 = 0x40000000;

//Peripheral memory map
pub const APB1PERIPH_BASE: u32 = PERIPH_BASE;
pub const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x00010000;
pub const AHB1PERIPH_BASE: u32 = PERIPH_BASE + 0x00020000;
pub const AHB2PERIPH_BASE: u32 = PERIPH_BASE + 0x08000000;

pub const RCC_BASE: u32 = AHB1PERIPH_BASE + 0x1000;
//AHB2 peripherals
pub const GPIOA_BASE: u32 = AHB2PERIPH_BASE;
pub const GPIOB_BASE: u32 = AHB2PERIPH_BASE + 0x0400;
pub const GPIOC_BASE: u32 = AHB2PERIPH_BASE + 0x0800;
pub const GPIOD_BASE: u32 = AHB2PERIPH_BASE + 0x0C00;
pub const GPIOE_BASE: u32 = AHB2PERIPH_BASE + 0x1000;
pub const GPIOF_BASE: u32 = AHB2PERIPH_BASE + 0x1400;
pub const GPIOG_BASE: u32 = AHB2PERIPH_BASE + 0x1800;
pub const GPIOH_BASE: u32 = AHB2PERIPH_BASE + 0x1C00;
pub const RCC_AHB2ENR: u32 = 0x4C; // RCC AHB2 peripheral clock enable register, Address offset: 0x4C

//********************  Bit definition for RCC_AHB2ENR register  **************
pub const RCC_AHB2ENR_GPIOAEN: u32 = 0x00000001;
pub const RCC_AHB2ENR_GPIOBEN: u32 = 0x00000002;
pub const RCC_AHB2ENR_GPIOCEN: u32 = 0x00000004;
pub const RCC_AHB2ENR_GPIODEN: u32 = 0x00000008;
pub const RCC_AHB2ENR_GPIOEEN: u32 = 0x00000010;
pub const RCC_AHB2ENR_GPIOFEN: u32 = 0x00000020;
pub const RCC_AHB2ENR_GPIOGEN: u32 = 0x00000040;
pub const RCC_AHB2ENR_GPIOHEN: u32 = 0x00000080;
pub const RCC_AHB2ENR_OTGFSEN: u32 = 0x00001000;
pub const RCC_AHB2ENR_ADCEN: u32 = 0x00002000;
pub const RCC_AHB2ENR_RNGEN: u32 = 0x00040000;

// *****************************************************************************
//
//                            General Purpose I/O
//
// *****************************************************************************
pub const GPIO_MODER: u32 = 0x00; /* GPIO port mode register,               Address offset: 0x00 */
pub const GPIO_OTYPER: u32 = 0x04; /* GPIO port output type register,        Address offset: 0x04 */
pub const GPIO_OSPEEDR: u32 = 0x08; /* GPIO port output speed register,       Address offset: 0x08 */
pub const GPIO_PUPDR: u32 = 0x0C; /* GPIO port pull-up/pull-down register,  Address offset: 0x0C */
pub const GPIO_IDR: u32 = 0x10; /* GPIO port input data register,         Address offset: 0x10 */
pub const GPIO_ODR: u32 = 0x14; /* GPIO port output data register,        Address offset: 0x14 */
pub const GPIO_BSRR: u32 = 0x18; /* GPIO port bit set/reset register,      Address offset: 0x18 */
pub const GPIO_LCKR: u32 = 0x1C; /* GPIO port configuration lock register, Address offset: 0x1C */
pub const GPIO_AFR0: u32 = 0x20; /* GPIO alternate function registers,     Address offset: 0x20-0x24 */
pub const GPIO_AFR1: u32 = 0x24; /* GPIO alternate function registers,     Address offset: 0x20-0x24 */
pub const GPIO_BRR: u32 = 0x28; /* GPIO Bit Reset register,               Address offset: 0x28 */
pub const GPIO_ASCR: u32 = 0x2C; /* GPIO analog switch control register,   Address offset: 0x2C */

// ******************  Bits definition for GPIO_MODER register  ****************
// GPIO_MODER register bit definitions
pub const GPIO_MODER_MODER0_0: u32 = 0x00000001;
pub const GPIO_MODER_MODER0_1: u32 = 0x00000002;
pub const GPIO_MODER_MODER1_0: u32 = 0x00000004;
pub const GPIO_MODER_MODER1_1: u32 = 0x00000008;
pub const GPIO_MODER_MODER2_0: u32 = 0x00000010;
pub const GPIO_MODER_MODER2_1: u32 = 0x00000020;
pub const GPIO_MODER_MODER3_0: u32 = 0x00000040;
pub const GPIO_MODER_MODER3_1: u32 = 0x00000080;
pub const GPIO_MODER_MODER4_0: u32 = 0x00000100;
pub const GPIO_MODER_MODER4_1: u32 = 0x00000200;
pub const GPIO_MODER_MODER5_0: u32 = 0x00000400;
pub const GPIO_MODER_MODER5_1: u32 = 0x00000800;
pub const GPIO_MODER_MODER6_0: u32 = 0x00001000;
pub const GPIO_MODER_MODER6_1: u32 = 0x00002000;
pub const GPIO_MODER_MODER7_0: u32 = 0x00004000;
pub const GPIO_MODER_MODER7_1: u32 = 0x00008000;
pub const GPIO_MODER_MODER8_0: u32 = 0x00010000;
pub const GPIO_MODER_MODER8_1: u32 = 0x00020000;
pub const GPIO_MODER_MODER9_0: u32 = 0x00040000;
pub const GPIO_MODER_MODER9_1: u32 = 0x00080000;
pub const GPIO_MODER_MODER10_0: u32 = 0x00100000;
pub const GPIO_MODER_MODER10_1: u32 = 0x00200000;
pub const GPIO_MODER_MODER11_0: u32 = 0x00400000;
pub const GPIO_MODER_MODER11_1: u32 = 0x00800000;
pub const GPIO_MODER_MODER12_0: u32 = 0x01000000;
pub const GPIO_MODER_MODER12_1: u32 = 0x02000000;
pub const GPIO_MODER_MODER13_0: u32 = 0x04000000;
pub const GPIO_MODER_MODER13_1: u32 = 0x08000000;
pub const GPIO_MODER_MODER14_0: u32 = 0x10000000;
pub const GPIO_MODER_MODER14_1: u32 = 0x20000000;
pub const GPIO_MODER_MODER15_0: u32 = 0x40000000;
pub const GPIO_MODER_MODER15_1: u32 = 0x80000000;

// GPIO_OTYPER register bit definitions
pub const GPIO_OTYPER_OT_0: u32 = 0x00000001;
pub const GPIO_OTYPER_OT_1: u32 = 0x00000002;
pub const GPIO_OTYPER_OT_2: u32 = 0x00000004;
pub const GPIO_OTYPER_OT_3: u32 = 0x00000008;
pub const GPIO_OTYPER_OT_4: u32 = 0x00000010;
pub const GPIO_OTYPER_OT_5: u32 = 0x00000020;
pub const GPIO_OTYPER_OT_6: u32 = 0x00000040;
pub const GPIO_OTYPER_OT_7: u32 = 0x00000080;
pub const GPIO_OTYPER_OT_8: u32 = 0x00000100;
pub const GPIO_OTYPER_OT_9: u32 = 0x00000200;
pub const GPIO_OTYPER_OT_10: u32 = 0x00000400;
pub const GPIO_OTYPER_OT_11: u32 = 0x00000800;
pub const GPIO_OTYPER_OT_12: u32 = 0x00001000;
pub const GPIO_OTYPER_OT_13: u32 = 0x00002000;
pub const GPIO_OTYPER_OT_14: u32 = 0x00004000;
pub const GPIO_OTYPER_OT_15: u32 = 0x00008000;

// GPIO_OSPEEDER register bit definitions
pub const GPIO_OSPEEDER_OSPEEDR0_0: u32 = 0x00000001;
pub const GPIO_OSPEEDER_OSPEEDR0_1: u32 = 0x00000002;
pub const GPIO_OSPEEDER_OSPEEDR1_0: u32 = 0x00000004;
pub const GPIO_OSPEEDER_OSPEEDR1_1: u32 = 0x00000008;
pub const GPIO_OSPEEDER_OSPEEDR2_0: u32 = 0x00000010;
pub const GPIO_OSPEEDER_OSPEEDR2_1: u32 = 0x00000020;
pub const GPIO_OSPEEDER_OSPEEDR3_0: u32 = 0x00000040;
pub const GPIO_OSPEEDER_OSPEEDR3_1: u32 = 0x00000080;
pub const GPIO_OSPEEDER_OSPEEDR4_0: u32 = 0x00000100;
pub const GPIO_OSPEEDER_OSPEEDR4_1: u32 = 0x00000200;
pub const GPIO_OSPEEDER_OSPEEDR5_0: u32 = 0x00000400;
pub const GPIO_OSPEEDER_OSPEEDR5_1: u32 = 0x00000800;
pub const GPIO_OSPEEDER_OSPEEDR6_0: u32 = 0x00001000;
pub const GPIO_OSPEEDER_OSPEEDR6_1: u32 = 0x00002000;
pub const GPIO_OSPEEDER_OSPEEDR7_0: u32 = 0x00004000;
pub const GPIO_OSPEEDER_OSPEEDR7_1: u32 = 0x00008000;
pub const GPIO_OSPEEDER_OSPEEDR8_0: u32 = 0x00010000;
pub const GPIO_OSPEEDER_OSPEEDR8_1: u32 = 0x00020000;
pub const GPIO_OSPEEDER_OSPEEDR9_0: u32 = 0x00040000;
pub const GPIO_OSPEEDER_OSPEEDR9_1: u32 = 0x00080000;
pub const GPIO_OSPEEDER_OSPEEDR10_0: u32 = 0x00100000;
pub const GPIO_OSPEEDER_OSPEEDR10_1: u32 = 0x00200000;
pub const GPIO_OSPEEDER_OSPEEDR11_0: u32 = 0x00400000;
pub const GPIO_OSPEEDER_OSPEEDR11_1: u32 = 0x00800000;
pub const GPIO_OSPEEDER_OSPEEDR12_0: u32 = 0x01000000;
pub const GPIO_OSPEEDER_OSPEEDR12_1: u32 = 0x02000000;
pub const GPIO_OSPEEDER_OSPEEDR13_0: u32 = 0x04000000;
pub const GPIO_OSPEEDER_OSPEEDR13_1: u32 = 0x08000000;
pub const GPIO_OSPEEDER_OSPEEDR14_0: u32 = 0x10000000;
pub const GPIO_OSPEEDER_OSPEEDR14_1: u32 = 0x20000000;
pub const GPIO_OSPEEDER_OSPEEDR15_0: u32 = 0x40000000;
pub const GPIO_OSPEEDER_OSPEEDR15_1: u32 = 0x80000000;

// GPIO_PUPDR register bit definitions
pub const GPIO_PUPDR_PUPDR0: u32 = 0x00000003;
pub const GPIO_PUPDR_PUPDR0_0: u32 = 0x00000001;
pub const GPIO_PUPDR_PUPDR0_1: u32 = 0x00000002;
pub const GPIO_PUPDR_PUPDR1: u32 = 0x0000000C;
pub const GPIO_PUPDR_PUPDR1_0: u32 = 0x00000004;
pub const GPIO_PUPDR_PUPDR1_1: u32 = 0x00000008;
pub const GPIO_PUPDR_PUPDR2: u32 = 0x00000030;
pub const GPIO_PUPDR_PUPDR2_0: u32 = 0x00000010;
pub const GPIO_PUPDR_PUPDR2_1: u32 = 0x00000020;
pub const GPIO_PUPDR_PUPDR3: u32 = 0x000000C0;
pub const GPIO_PUPDR_PUPDR3_0: u32 = 0x00000040;
pub const GPIO_PUPDR_PUPDR3_1: u32 = 0x00000080;
pub const GPIO_PUPDR_PUPDR4: u32 = 0x00000300;
pub const GPIO_PUPDR_PUPDR4_0: u32 = 0x00000100;
pub const GPIO_PUPDR_PUPDR4_1: u32 = 0x00000200;
pub const GPIO_PUPDR_PUPDR5: u32 = 0x00000C00;
pub const GPIO_PUPDR_PUPDR5_0: u32 = 0x00000400;
pub const GPIO_PUPDR_PUPDR5_1: u32 = 0x00000800;
pub const GPIO_PUPDR_PUPDR6: u32 = 0x00003000;
pub const GPIO_PUPDR_PUPDR6_0: u32 = 0x00001000;
pub const GPIO_PUPDR_PUPDR6_1: u32 = 0x00002000;
pub const GPIO_PUPDR_PUPDR7: u32 = 0x0000C000;
pub const GPIO_PUPDR_PUPDR7_0: u32 = 0x00004000;
pub const GPIO_PUPDR_PUPDR7_1: u32 = 0x00008000;
pub const GPIO_PUPDR_PUPDR8: u32 = 0x00030000;
pub const GPIO_PUPDR_PUPDR8_0: u32 = 0x00010000;
pub const GPIO_PUPDR_PUPDR8_1: u32 = 0x00020000;
pub const GPIO_PUPDR_PUPDR9: u32 = 0x000C0000;
pub const GPIO_PUPDR_PUPDR9_0: u32 = 0x00040000;
pub const GPIO_PUPDR_PUPDR9_1: u32 = 0x00080000;
pub const GPIO_PUPDR_PUPDR10: u32 = 0x00300000;
pub const GPIO_PUPDR_PUPDR10_0: u32 = 0x00100000;
pub const GPIO_PUPDR_PUPDR10_1: u32 = 0x00200000;
pub const GPIO_PUPDR_PUPDR11: u32 = 0x00C00000;
pub const GPIO_PUPDR_PUPDR11_0: u32 = 0x00400000;
pub const GPIO_PUPDR_PUPDR11_1: u32 = 0x00800000;
pub const GPIO_PUPDR_PUPDR12: u32 = 0x03000000;
pub const GPIO_PUPDR_PUPDR12_0: u32 = 0x01000000;
pub const GPIO_PUPDR_PUPDR12_1: u32 = 0x02000000;
pub const GPIO_PUPDR_PUPDR13: u32 = 0x0C000000;
pub const GPIO_PUPDR_PUPDR13_0: u32 = 0x04000000;
pub const GPIO_PUPDR_PUPDR13_1: u32 = 0x08000000;
pub const GPIO_PUPDR_PUPDR14: u32 = 0x30000000;
pub const GPIO_PUPDR_PUPDR14_0: u32 = 0x10000000;
pub const GPIO_PUPDR_PUPDR14_1: u32 = 0x20000000;
pub const GPIO_PUPDR_PUPDR15: u32 = 0xC0000000;
pub const GPIO_PUPDR_PUPDR15_0: u32 = 0x40000000;
pub const GPIO_PUPDR_PUPDR15_1: u32 = 0x80000000;
