#![no_main]
#![no_std]

extern crate panic_halt;
use cortex_m;
use cortex_m_rt::entry;
use stm32f1xx_hal::{stm32, prelude::* };
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    // 
    let cp = cortex_m::Peripherals::take().unwrap();
    
    // device peripherals
    let dp = stm32::Peripherals::take().unwrap();
    
    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut delay = stm32f1xx_hal::delay::Delay::new(cp.SYST, clocks);

    loop {
        delay.delay_ms(500_u32);
        led.set_high().unwrap();
        delay.delay_ms(500_u32);
        led.set_low().unwrap();
    }

}
