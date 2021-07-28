pub use stm32f3_discovery::stm32f3xx_hal;
use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::stm32;

/// initializes the device peripherals and grant access to work with LED pins
pub fn init() {

    // initializing device peripherals to access led's pins
    let device_peripherals = stm32::Peripherals::take().unwrap();
    let mut rcc = device_peripherals.RCC.constrain();

    // initializes GPIOE
    let mut gpioe = device_peripherals.GPIOE.split(&mut rcc.ahb);

    // configures led 3
    gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    // configures led 4
    gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

}
