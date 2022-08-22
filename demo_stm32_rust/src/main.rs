//! Controlling a servo using Rust.
//!
//! This demo uses an stm32 "blue pill", with the servo's control wire connected to pin A0.
//!
//! The blue pill board can be powered either via the SWD debugger, or its 3.3V or 5V pins.  The
//! servo needs between 4.8-6V of power, and more current than the debugger or board can provide,
//! so you should hook it up to a power supply that can provide it.
//!
//! The current demo has the blue pill powered via the SWD debugger through USB, and the servo
//! powered separately via 4x 1.5V AA batteries in a pack.
//! Both share a ground pin.
//!
//! To see the servo signal, hook up the oscilloscope to the same signal pin (A0). It's most
//! conveninent to do this through a common row on a breadboard.

// Disable the standard library (which needs allocators and all sorts of things which embedded
// environments don't have. Also tell the compiler not to run doctests, as we need this to be
// cross-compiled and it won't work, and that we aren't providing a standard main() function.
#![no_std]
#![cfg_attr(not(doc), no_main)]

// This is the "Real Time Terminal" support for the debugger. I'm using an ST-Link V2 clone.
// This gives us the ability to print out stack traces and do logging to the St-Link V2, and
// then the console.
use rtt_target::{rtt_init_print};
use panic_rtt_target as _;

// The Blue Pill's HAL crate imports.
use stm32f1xx_hal::{
    prelude::*,
    pac,
    time::ms,
    timer::{Channel, Tim2NoRemap},
};
use cortex_m_rt::entry;


#[entry]
fn main() -> ! {
    // Init buffers for debug printing.
    rtt_init_print!();

    // START of board setup and configuring of system things Get access to the core peripherals
    // from the cortex-m crate (stm32 basics). This denies access to anyone else.
    // We'll also configure the build-in LED so we can show liveness by blinking it.
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate.  So things
    // on the blue pill board.
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs. These are steps towards further configuration.
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = cp.SYST.delay(&clocks);

    // Acquire the GPIO A, and C peripherals (we dont need peripherals from the B GPIO block).
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut afio = dp.AFIO.constrain();
    // END of board setup

    
    // Set up PWM, controlled by the TIM2 timer.
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    // NOTE, we're setting up two pins, but just using one. This seems to be a weakness of the
    // library we use to set up the board.
    let pins_block_one = (c1, c2);  // Not using c3 (pa2), c4 (pa3)
    let mut pwm_tim2 = dp.TIM2.pwm_hz::<Tim2NoRemap, _, _>(pins_block_one, &mut afio.mapr, 1.kHz(), &clocks);

    pwm_tim2.enable(Channel::C1);
    pwm_tim2.set_period(ms(20).into_rate());

    let max_duty = pwm_tim2.get_max_duty();
    let max = max_duty/10; // 2ms on
    let min = max_duty/20; // 1ms on
    let one_degree = (max - min) / 180;

    loop {
        // Blink the Blue Pill's onboard LED to show liveness.
        led.set_high();
        
        // Move the first servos from min to max range, the second three half their range
        // to make the movements of the base a bit less violent
        for degrees in 0..=180 {
            delay.delay_ms(10_u16);
            pwm_tim2.set_duty(Channel::C1, min + degrees * one_degree);
        }

        led.set_low();

        // Move the servos from max to min range
        for degrees in 0..=180 {
            delay.delay_ms(10_u16);
            pwm_tim2.set_duty(Channel::C1, min + (180 - degrees) * one_degree);
        }
    }
}
