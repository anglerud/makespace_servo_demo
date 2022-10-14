# Servo demo

A demo of controlling RC servos from Rust.

We have two different servos to show, a small MG90s, and a larger 6221MG. These
are both RC servos, and are controlled in exactly the same way.


## What *are* RC servos?

These servos rotate to set angles based on the signal we send. The demo is
going to move the servos through a 120 or 180 degree movement depending on the
servo we select.

Servos have different ranges of motion - from 90°, through 180°, or even more.
Each servo has a spec sheet which will tell you its range of motion, among
other things.

An "RC" servo is a common type of hobby servo - the RC is for "Radio Control",
and the name comes from their use back in the 1960s for radio control airplanes
and similar. It was also at that time the control system (described below) was
invented. Now these servos are used in all sorts of general hobby projects.

The servo is made up of a small electric motor, a gearbox to lower the speed
and increase the torque, a potentiometer connected to the output shaft, and
some control electronics which interpret the signal to control the motor so
that the desired output shaft angle is achieved. The output shaft potentiometer
is how the servo can know what angle it's achieved vs what's been requested.

On the drive shaft are optional attachments known as "servo horns", that make
it easy to attach the servo to the thing you'd like to move. The servo shaft
also accepts a screw through its center as another way of attaching it to
things.


# The servos

<a href="images/servo_comparison_large.jpg"><img src="images/servo_comparison_small.jpg" /></a>

We have two different servos. The immediately obvious difference is that the
6221MG is larger than the MG90s. It's also nearly 10x as powerful - see the
"stall torque" numbers below. Another difference is that the smaller servo
moves through 180 degrees, and the larger through 120 degrees (60 degrees
either side of neutral).


## MG90s

<a href="images/mg90s_large.jpg"><img src="images/mg90s_small.jpg" /></a>


[Data Sheet](https://datasheetspdf.com/pdf/1106582/ETC/MG90S/1)

Period of 20ms, and a duty cycle of 1-2ms.  Rotates 90deg in each direction -
180deg in total.

    Weight: 13.4 g
    Dimension: 22.5 x 12 x 35.5 mm approx
    Stall torque: 1.8 kgf·cm (4.8V ), 2.2 kgf·cm (6 V)
    Operating speed: 0.1 s/60 degree  (4.8 V), 0.08 s/60 degree (6 V)
    Operating voltage: 4.8 V - 6.0 V
    Dead band width: 5 μs


## PDI-6221MG

<a href="images/pdi6221mg_large.jpg"><img src="images/pdi6221mg_small.jpg" /></a>


[Link](https://www.rcgoing.com/jx-pdi-6221mg-20kg-large-torque-digital-standard-servo-360-degree-cw/)

Period of 20ms, and duty cycle of 1-2ms. Rotates 60deg in each direction -
120deg total.

    Model No: PDI-6221MG
    Voltage: 4.8~6V
    Stall Torque (4.8V): 17.25kg
    Stall Torque (6.0V): 20.32kg
    Speed (4.8V): 0.18sec/60°
    Speed (6.0V): 0.16sec/60°
    Dead Band: 2µs 1520µ/330hz
    Motor: Std cored
    Gears: Metal
    Spline Count: 25T
    Bearing: 2BB
    Dimensions: 40.5 x 20.2 x 38mm
    Weight: 62g
    Connector: JR type
    Servo Wire Length: 265mm


A note on the stall torque and operating speed. Using this servo as an example,
it says that at 4.8V, it can turn the output shaft 60 degrees in 0.18 seconds.
When doing so - it can lift up to a 17.25kg weight that is attached at a radius
of 1cm from the center of the output shaft.


# Connecting a servo

The servo connectivity is through three wires that are terminated with a set of
0.1" jacks. The order of the jacks is commonly signal, voltage, ground and are
also color coded. There are two differnet schemes:

* black, red, white
* brown, red, yellow/orange

where:

* brown or black is ground.
* red is servo power.
* white, yellow, or orange is signal.

Power is commonly 4.8V, but as you can see from the specifications above, many
accept a range of voltages, and they're even able to deal with lower voltages
than specified for the control signal. For example, we'll give the control
signal at 3.3V with some of our microcontrollers, and you'll be able to still
see the servo respond.

We'll power the board from a different power supply than the servo. The servo
is getting 6V from a battery pack, and or microcontroller will get either ~3V
from a battery pack, or be fed via USB. When using the stm32, it'll gets 3.3V
from the St-link programmer, and when using the Arduino Nano we'll plug it into
USB directly. Both can also be powered with either 3.3V or 5V on one of the
pins marked accordingly. In the diagram I represent "generic power to the
microcontroller" with a 3V battery pack, but in reality it's probably USB.

We're connecting an oscilloscope so that we can see the signal, and we're using
a breadboard to keep things a little neater. The end result should look like
this:

<a href="images/ardunino_setup_large.jpg"><img src="images/arduino_setup_small.jpg" /></a>

This is the same as a drawing:
<a href="images/arduino_servo_bb.png"><img src="images/arduino_servo_bb_small.png" /></a>

Note that while the microcontroller can also *output* either 3.3V or 5V on
those pins, they won't provide enough current to move the servo, and trying to
do so can damage the microcontroller board.


# Controlling the servo

The servo expects to receive a pulse each 20ms (so a 50HZ Signal), where the
width of the pulse determines the position of the output shaft. At a 1ms width,
the servo is at one extreme, and at 2ms it is at the opposite extreme. At
1.5ms, the servo is at its neutral (middle) position.

The signal is expected to be at 4.8V, but most servos will happily consume
3.3V, all the way up to its maximum operating voltage.

The above is the most common. However, some 180° servos rotate through 90° with
a 1ms difference in pulse widths - and so to get them to rotate though the full
range - you need to extend the pulse width range from 0.5ms to 2.5ms. Hopefully
the spec sheet will inform you of this, but sometimes you may have to
experiment to see.


# Other kinds of servos

Other servos may operate at different voltages, have control schemes where the
position can be read back, or even have programmable movement profiles.

There are also continuously rotating servos, and servos which respond to just
the %age time the signal is active, rather than the 50HZ signal that these RC
servos use.


# Running the demos

## Use a Servo tester

The easiest way to see the servo in action is to use a servo tester!  There are
many available, for example [this
model](https://www.amazon.co.uk/VIPMOON-Consistency-Connection-Protection-Centering/dp/B07T17LB2B/ref=sr_1_11)
available cheaply via Amazon.

In the picture below you can see the model we have in the makespace hooked up:

<a href="images/servo_tester_setup_large.jpg"><img src="images/servo_tester_setup_small.jpg" /></a>


When you turn the knob, the servo turns, it's very easy. I've also hooked it up
to the oscilloscope so you can see the pulses that it's sending:

![servo tester oscilloscope capture](images/servo_tester_oscilloscope.png)

What isn't pictured there is that our tester actually does the wrong thing.
When the pulses are lengthened, the total period of the cycle is extended -
which shouldn't be the case. However, the servos are very robust and handle
that condition as well.


## STM32 And rust

### Physical setup for the STM32

We connect the:

* Battery pack's ground to the black or blue power rail on the breadboard.
* Servo's power lead (red) to the battery pack's 6V output.
* Servo's ground lead (black or brown) to the common ground on the breadboard.
* Servo's signal lead (white, yellow, or orange) to a row on the breadboard.
* Microcontroller's pin PA0 to the same row on the breadboard as the servo
  signal lead.
* Microcontroller's SWD pins to the ST-link V2 programmer.
* Oscilloscope's ground probe to the breadboard's ground rail.
* Oscilloscope's signal probe to the same row on the breadboard as the servo
  signal lead.

We have connected up the oscilloscope so that we can visualise the servo
signal.

NOTE: The PWM pin we have selected is PA0 on the blue pill microcontroller
board. The PWM output is 3.3V, so below the 4.8V that the servo is expecting,
but as you'll see, it works fine.

<!-- TODO: repeat the stm32 images here -->


### Setup

To prepare - go to https://rustup.rs/ and follow the instructions to get Rust
installed.

Add the cross-compilation target for the microcontroller:

    rustup target add thumbv7m-none-eabi

Install `probe-rs`'s dependencies with

    sudo apt install -y libusb-1.0-0-dev libftdi1-dev libudev-dev

and then `probe-rs` itself with

    cargo install probe-run

Set up the `udev` rules so that you can access the st-link programmer without
having to use `sudo` or being the root user. Create the file
`/etc/udev/rules.d/70-st-link.rules` and add the contents:

    # STM32F3DISCOVERY rev A/B - ST-LINK/V2
    ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", TAG+="uaccess"

    # STM32F3DISCOVERY rev C+ - ST-LINK/V2-1
    ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", TAG+="uaccess"

Reload all the `udev` rules with:

    sudo udevadm control --reload-rules

Then plug in the St-Link V2 programmer.


### Build, and flash the code

We flash the code onto the blue pill with `cargo run`, which will invoke the
`probe-run` launcher, part of the probe.rs tool chain. This is configured via
Cargo, the Rust package manager and build system. See
`demo_stm32_rust/.cargo/config`.

Note that `probe-run` also connects the debugger, so we'll be able to get debug
prints and even stack traces from the code.

Once flashed of course, the microcontroller operates without needing the
programmer. We only use it to power the board after the first flash.

You'll see the servo move between its two extremes continuously. You'll also be
able to see the control signal on the oscilloscope - and you can see what
signal corresponds to what angle of the servo.

Here is what is displayed on our oscilloscope:

![stm32 oscilloscope capture](images/stm32_oscilloscope.png)


### probe-run

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

## Micro:bit and python

The [BBC Micro:bit](https://microbit.org/) can be easily used to control the
servo as well. You can use the online IDE to generate the code to flash to the
device.

Head to the [IDE](https://python.microbit.org/v/beta) at microbit.org, and use
the code below:


```python
from microbit import * 
# Servo control: 
# 50 = ~1 millisecond pulse all right 
# 75 = ~1.5 millisecond pulse center 
# 100 = ~2.0 millisecond pulse all left 
pin0.set_analog_period(20)

while True: 
	pin0.write_analog(75)
	sleep(1000)
	pin0.write_analog(50)
	sleep(1000)
	pin0.write_analog(100)
	sleep(1000)
```

This snippet is from the excellent micro:bit [article on
servos](https://support.microbit.org/support/solutions/articles/19000101864-using-a-servo-with-the-micro-bit).

This demo sends the RC control signal on pin 0. We need to connect the
microcontroller's ground to a common ground and then power the micro:bit, which
we do via the onboard micro-USB port. It should look like this:

XXX: No, that's the wrong picture, that's the Arduino nano! Rename and replace that!
<a href="images/microbit_setup_large.jpg"><img src="images/microbit_setup_small.jpg" /></a>

<!--
This is the same as a drawing:
<a href="images/stm32_servo_bb.png"><img src="images/stm32_servo_bb_small.png" /></a>

And the same as a schematic:
<a href="images/stm32_servo_schem.png"><img src="images/stm32_servo_schem_small.png" /></a>
--> 

![microbit oscilloscope capture](images/microbit_oscilloscope.png)



If you're using Google Chrome as your browser, you can connect the microbit to
a USB port on your computer and click "Send to micro:bit". If you're using
another browser, you can select "Save" then drag the resulting downloaded file
over to the microbit with your file manager.


## Arduino nano and c++

Here we've wired up our Arduino Nano to the same servo setup. The contol pin
is "D9".

<a href="images/arduino_setup_large.jpg"><img src="images/arduino_setup_small.jpg" /></a>

We need to import the `Servo` library into your Arduino IDE. Different versions
of the Arduino IDE unfortunately have the library manager in different places.
Try either:

```
Tools -> Manage Libraries.
Find the "Servo" library by "Arduino", and install that.
```

or

```
Sketch -> Import Library -> Servo
```

Now add the following code into your sketch, and upload it. If you're using a
Nano, like we are here, remember to select the Nano from Tools -> Board, and
pick the one with the ATMega328. You may also have to select which serial port
to use.


```C++
#include <Servo.h>

Servo myservo;  // create servo object to control a servo
// twelve servo objects can be created on most boards

int pos = 0;    // variable to store the servo position

void setup() {
  myservo.attach(9);  // attaches the servo on pin 9 to the servo object
}

void loop() {
  for (pos = 0; pos <= 180; pos += 1) { // goes from 0 degrees to 180 degrees
    // in steps of 1 degree
    myservo.write(pos);              // tell servo to go to position in variable 'pos'
    delay(15);                       // waits 15ms for the servo to reach the position
  }
  for (pos = 180; pos >= 0; pos -= 1) { // goes from 180 degrees to 0 degrees
    myservo.write(pos);              // tell servo to go to position in variable 'pos'
    delay(15);                       // waits 15ms for the servo to reach the position
  }
}
```

When you run it, your servo should move - and you should see the following on your
oscilloscope.

![arduino oscilloscope capture](images/arduino_oscilloscope.png)


# Licenses


Images and text licensed under a <a rel="license"
href="http://creativecommons.org/licenses/by-sa/4.0/">Creative Commons
Attribution-ShareAlike 4.0 International License</a>.
<a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/80x15.png" /></a>
