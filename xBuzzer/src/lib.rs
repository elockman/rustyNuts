use tokio::time::{sleep, Duration};
use sysfs_gpio::{Direction, Pin};

pub struct xBuzzer {
    pin: Pin,
}

impl Buzzer {
    pub fn new(pin: u64) -> Self {
        let pin = Pin::new(pin);
        pin.export().unwrap();
        pin.set_direction(Direction::Out).unwrap();
        Buzzer { pin }
    }

    pub async fn task(&self, frequency: u64, alarm_duration_ms: u64, dwell_duration_ms: u64, num_alarms: u64) {
        let period = Duration::from_millis(1000 / frequency);
        let half_period = period / 2;
        let alarm_duration = Duration::from_millis(alarm_duration_ms);
        let dwell_duration = Duration::from_millis(dwell_duration_ms);

        for _ in 0..num_alarms {
            let mut elapsed = Duration::ZERO;

            while elapsed < alarm_duration {
                self.pin.set_value(1).unwrap();
                sleep(half_period).await;
                self.pin.set_value(0).unwrap();
                sleep(half_period).await;
                elapsed += period;
            }

            sleep(dwell_duration).await;
        }
    }
}

impl Drop for xBuzzer {
    fn drop(&mut self) {
        self.pin.unexport().unwrap();
    }
}
