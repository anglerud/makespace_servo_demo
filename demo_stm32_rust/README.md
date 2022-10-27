# A demo of controlling RC servos from Rust.

## Setup 

Connect the:

* battery pack's ground to the black or blue power rail on the breadboard.
* servo's power lead (red) to the battery pack's 6V output.
* servo's ground lead (black or brown) to the common ground on the breadboard.
* servo's signal lead (white, yellow, or orange) to a row on the breadboard.
* microcontroller's pin PA0 to the same row on the breadboard as the servo
  signal lead.
* microcontroller's SWD pins to the ST-link V2 programmer.
* oscilloscope's ground probe to the breadboards's ground rail.
* oscilloscope's signal probe to the same row on the breadboard as the servo
  signal lead.

We have connected up the oscilloscope so that we can visualise the servo
signal. If you don't have an oscilloscope, just leave this part out.

NOTE: The PWM pin we have selected is PA0 on the blue pill microcontroller
board. The PWM output is 3.3V, so below the 4.8V that the servo is expecting,
but as you'll see, it works fine.


## Running the demo

We flash the code onto the blue pill with `cargo run`, which will invoke the
`probe-run` programmer, part of the probe.rs toolchain. This is configured via
Cargo, the Rust package manager and build system.

Note that `probe-run` also connects the debugger, so we'll be able to get debug
prints and even stack traces from the code.

Once flashed of course, the microcontroller operates without needing the
programmer. We only use it to power the board after the first flash.

You'll see the servo move between its two extremes continuousy. You'll also be
able to see the control signal on the oscilloscope - and you can see what
signal corresponds to what angle of the servo.


## probe-run

The `probe-run` configuration, and some other building and linking information
is in the `.cargo/config` file - you can take a look in there, it is well
commented. More information on `probe-run` can be found at
[probe-run](https://github.com/knurling-rs/probe-run), from the excellent
Knurling project - it uses tooling from [probe.rs](https://probe.rs/).

If you run `cargo build`, `probe-run` won't be built - only the binary which
would have ended up on the microcontroller. You can built it and take a look at it:

```
❯ file target/thumbv7m-none-eabi/debug/servo_demo
target/thumbv7m-none-eabi/debug/servo_demo: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, with debug_info, not stripped
```
