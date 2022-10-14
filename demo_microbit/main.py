"""
Servo control: 
50 = ~1 millisecond pulse all right 
75 = ~1.5 millisecond pulse center 
100 = ~2.0 millisecond pulse all left 
"""
from microbit import * 

pin0.set_analog_period(20)


while True: 
	pin0.write_analog(75)
	sleep(1000)
	pin0.write_analog(50)
	sleep(1000)
	pin0.write_analog(100)
	sleep(1000)
