use embedded_hal::digital::{InputPin, OutputPin};
use esp_hal::delay::Delay;
use loadcell::{hx711::HX711, LoadCell};

pub struct Loadcell<SckPin, DTPin> {
    hx711: HX711<SckPin, DTPin, Delay>,
    last_read: f32,
}

impl<SckPin, DTPin> Loadcell<SckPin, DTPin>
where
    SckPin: OutputPin,
    DTPin: InputPin,
{
    pub fn new(sck_pin: SckPin, dt_pin: DTPin, delay: Delay) -> Self {
        let mut hx711 = HX711::new(sck_pin, dt_pin, delay);
        hx711.tare(16);
        hx711.set_scale(1.0);
        Loadcell {
            hx711,
            last_read: 0.0,
        }
    }

    pub fn read_scaled(&mut self) -> f32 {
        if self.hx711.is_ready() {
            let reading = self.hx711.read_scaled();
            if let Ok(x) = reading {
                self.last_read = x;
            }
        }
        self.last_read
    }
}