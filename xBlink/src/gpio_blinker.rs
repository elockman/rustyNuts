use tokio::time::{sleep, Duration};
use sysfs_gpio::{Direction, Pin};

pub struct GpioBlinker {
    pin: Pin,
}

impl GpioBlinker {
    pub fn init(pin_number: u64) -> Self {
        let pin = Pin::new(pin_number);
        pin.export().unwrap();
        pin.set_direction(Direction::Out).unwrap();
        GpioBlinker { pin }
    }

    pub async fn task(&self, on_time_ms: u64, off_time_ms: u64, num_blinks: u64) {
        let on_duration = Duration::from_millis(on_time_ms);
        let off_duration = Duration::from_millis(off_time_ms);

        let mut blinks = 0;
        while num_blinks == 0 || blinks < num_blinks {
            self.pin.set_value(1).unwrap();
            sleep(on_duration).await;
            self.pin.set_value(0).unwrap();
            sleep(off_duration).await;
            blinks += 1;
        }
    }
}

impl Drop for GpioBlinker {
    fn drop(&mut self) {
        self.pin.unexport().unwrap();
    }
}
