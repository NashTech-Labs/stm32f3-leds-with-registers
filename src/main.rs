#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_itm;
use cortex_m_rt::entry;
use stm32f3_leds_with_registers::configuration::initialize::init;
use cortex_m::asm::delay;


#[entry]
/// This program pertains to blink STM32F3's LEDs using register's address
fn main() -> ! {

    // initializing device peripherals to access led's pins
    init();

    loop {
        unsafe {
            // stores GPIO's BSRR(Bit Set Reset Register) address
            const GPIOE_BSRR: u32 = 0x48001018;

            // Turns on LD3 by setting 9th bit
            *(GPIOE_BSRR as *mut u32) = 1 << 9;

            // adding delay for a moment using CPU cycles
            delay(1_000_000_u32);

            // Turns off LD3 by setting 25th bit
            *(GPIOE_BSRR as *mut u32) = 1 << 25;

            // adding delay for a moment using CPU cycles
            delay(1_000_000_u32);

            // Turns on LD4 by setting 4th bit
            *(GPIOE_BSRR as *mut u32) = 1 << 8;

            // adding delay for a moment using CPU cycles
            delay(1_000_000_u32);

            // Turns off LD4 by setting 24th bit
            *(GPIOE_BSRR as *mut u32) = 1 << 24;

            // adding delay for a moment using CPU cycles
            delay(1_000_000_u32);
        }
    }
}

