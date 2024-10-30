mod gpio_blinker;

use gpio_blinker::GpioBlinker;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let gpio_blinker = GpioBlinker::init(128); // Initialize GPIO pin 128 (CHANGE FOR YOUR APPLICATION)
    gpio_blinker.task(2500, 2500, 10).await;    // Blink with 2500ms on, 2500ms off, for 10 blinks
}
