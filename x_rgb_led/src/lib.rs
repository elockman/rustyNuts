use tokio::time::{sleep, Duration};
use sysfs_gpio::{Direction, Pin};

pub struct RgbLed {
    r_pin: Pin,
    g_pin: Pin,
    b_pin: Pin,
}

impl RgbLed {
    pub fn init(r_pin: u64, g_pin: u64, b_pin: u64) -> Self {
        let r_pin = Pin::new(r_pin);
        let g_pin = Pin::new(g_pin);
        let b_pin = Pin::new(b_pin);

        r_pin.export().unwrap();
        g_pin.export().unwrap();
        b_pin.export().unwrap();

        r_pin.set_direction(Direction::Out).unwrap();
        g_pin.set_direction(Direction::Out).unwrap();
        b_pin.set_direction(Direction::Out).unwrap();

        RgbLed { r_pin, g_pin, b_pin }
    }

    pub async fn task(
        &self,
        r_max: u8,
        g_max: u8,
        b_max: u8,
        time_on: u64,
        time_off: u64,
        blinks: u64,
        fade_freq: u64,
    ) {
        if time_on != 0 && time_off != 0 {
            self.blink(r_max, g_max, b_max, time_on, time_off, blinks).await;
        } else if fade_freq != 0 {
            self.fade(r_max, g_max, b_max, fade_freq).await;
        } else {
            self.set_color(r_max, g_max, b_max).await;
        }
    }

    async fn blink(
        &self,
        r_max: u8,
        g_max: u8,
        b_max: u8,
        time_on: u64,
        time_off: u64,
        blinks: u64,
    ) {
        let mut blink_count = 0;
        while blinks == 0 || blink_count < blinks {
            self.set_color(r_max, g_max, b_max).await;
            sleep(Duration::from_millis(time_on)).await;
            self.set_color(0, 0, 0).await;
            sleep(Duration::from_millis(time_off)).await;
            blink_count += 1;
        }
    }

    async fn fade(&self, r_max: u8, g_max: u8, b_max: u8, fade_freq: u64) {
        loop {
            for i in 0..=255 {
                self.set_color(
                    (r_max as u16 * i / 255) as u8,
                    (g_max as u16 * i / 255) as u8,
                    (b_max as u16 * i / 255) as u8,
                ).await;
                sleep(Duration::from_millis(fade_freq)).await;
            }
            for i in (0..=255).rev() {
                self.set_color(
                    (r_max as u16 * i / 255) as u8,
                    (g_max as u16 * i / 255) as u8,
                    (b_max as u16 * i / 255) as u8,
                ).await;
                sleep(Duration::from_millis(fade_freq)).await;
            }
        }
    }

    async fn set_color(&self, r: u8, g: u8, b: u8) {
        self.r_pin.set_value(if r > 0 { 1 } else { 0 }).unwrap();
        self.g_pin.set_value(if g > 0 { 1 } else { 0 }).unwrap();
        self.b_pin.set_value(if b > 0 { 1 } else { 0 }).unwrap();
    }
}

impl Drop for RgbLed {
    fn drop(&mut self) {
        self.r_pin.unexport().unwrap();
        self.g_pin.unexport().unwrap();
        self.b_pin.unexport().unwrap();
    }
}
