use x_rgb_led::RgbLed;

#[tokio::main]
async fn main() {
    let rgb_led = RgbLed::init(137, 142, 134); // Initialize RGB LED on pins 137, 142, and 134
    rgb_led.task(0, 0, 255, 0, 0, 0, 1).await; // Breathe blue 1Hz
}
