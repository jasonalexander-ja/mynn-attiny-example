ATtiny2313 Template
==

Rust project for the ATtiny2313.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build --release` to build the firmware.

3. Use an Arduino as an ISP to upload your firmware: 
   * Connect your AVR based Arduino to your PC, opening the Arduino IDE, select the correct port and Arduino board then open the example sketch File -> Examples -> 11.ArduinoISP -> ArduinoISP and upload it. 
   <img src="docs/ide.png">
   
   * Take note of what port your arduino is on 

4. Connect your arduino to your ATtiny2313:
   * Arduino pin 13 -> ATtiny2313 pin 19
   * Arduino pin 12 -> ATtiny2313 pin 18
   * Arduino pin 11 -> ATtiny2313 pin 17
   * Arduino pin 10 -> ATtiny2313 pin 1
   <img src="docs/circuit.png">

5. With `avrdude` installed, upload using the following command: 

```
avrdude -c arduino_as_isp -p t2313 -P [serial port of arduino] -U flash:w:"target/avr-attiny2313/release/[executable name].elf":a
```

