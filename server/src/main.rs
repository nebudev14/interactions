use rust_gpiozero::*;

fn main() {
    let mut led = LED::new(26);
    led.blink(2.0, 3.0);
    led.wait();
}
