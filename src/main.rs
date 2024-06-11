#![no_std]
#![no_main]

use panic_halt as _;

use mynn::network::{EndLayer, ProcessLayer};
use mynn::activations::SIGMOID;

const FIRST_LAYER_WEIGHTS: [[f32; 2]; 3] = [
    [-8.086764, -8.086563],
    [-10.876657, -10.877184],
    [10.14248, 10.143111]
];
const FIRST_LAYER_BIASES: [f32; 3] = [3.3848374, 4.80076, -15.381532];

const SECOND_LAYER_WEIGHTS: [[f32; 3]; 1] = [[-2.4123971, -6.627293, -8.613715]];
const SECOND_LAYER_BIASES: [f32; 1] = [4.3186426];

#[avr_device::entry]
fn main() -> ! {
    let dp = attiny_hal::Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    let mut led = pins.pb4.into_output();
    let in1 = pins.pb0.into_pull_up_input();
    let in2 = pins.pb1.into_pull_up_input();

    let mut network: ProcessLayer<3, 2, 1, ProcessLayer<1, 3, 1, EndLayer<1>>> = 
        ProcessLayer::new_with(
            ProcessLayer::new_with(EndLayer(), SECOND_LAYER_WEIGHTS, SECOND_LAYER_BIASES), 
            FIRST_LAYER_WEIGHTS, 
            FIRST_LAYER_BIASES
        );

    network.predict([1.0, 1.0], &SIGMOID);

    led.set_high();

    loop {
        let input1 = if in1.is_high() { 1.0f32 } else { 0.0f32 };
        let input2 = if in2.is_high() { 1.0f32 } else { 0.0f32 };

        let result = network.predict([input1, input2], &SIGMOID)[0];

        if result >= 0.9f32 { led.set_high(); } 
        if result <= 0.1 { led.set_low(); }
    }
}
