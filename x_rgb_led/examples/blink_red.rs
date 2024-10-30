use x_rgb_led::RgbLed;

#[tokio::main]
async fn main() {
    let rgb_led = RgbLed::init(137, 142, 134); // Initialize RGB LED on pins 137, 142, and 134
    rgb_led.task(255, 0, 0, 500, 500, 5, 0).await; // Blink red color with 500ms on, 500ms off, for 5 blinks
}
