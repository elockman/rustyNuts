mod gpio_blinker;

use gpio_blinker::GpioBlinker;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let gpio_blinker = GpioBlinker::init(128); // Initialize GPIO pin 128
    gpio_blinker.task(500, 500, 10).await;    // Blink with 500ms on, 500ms off, for 10 blinks
}
