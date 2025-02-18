use defmt::info;
use esp_hal::analog::adc::{Adc, AdcPin};
use esp_hal::gpio::GpioPin;
use esp_hal::peripherals::ADC1;
use nb::block;

pub struct Data {
    pub value: f32,
}

// https://github.com/plantineers/edge/blob/main/src/main.rs
/// A struct containing the hardware(via pointers to the right addresses of course) for the hw390 moisture sensor
pub struct Hw390<'a> {
    adc: Adc<'a, ADC1>,
    pin: AdcPin<GpioPin<3>, ADC1>,
}

impl<'a> Hw390<'a> {
    pub fn new(adc: Adc<'a, ADC1>, pin: AdcPin<GpioPin<3>, ADC1>) -> Self {
        Hw390 { adc, pin }
    }

    /// Read out moisture data from the hw390 sensor and normalise it
    pub fn read(&mut self) -> Data {
        let readout = block!(self.adc.read_oneshot(&mut self.pin)).unwrap();
        let value = (readout as f32); // as f32 * 3.3) / (4095.0);
        Data { value  }
    }
}

/// The hw390 moisture sensor returns a value between 3000 and 4095
/// From our measurements the sensor was in water at 3000 and in air at 4095
/// We want to normalise the values to be between 0 and 1, so that 1 is in water and 0 is in air
fn normalise_humidity_data(readout: u16) -> f32 {
    info!("HW390 readout: {}", readout);
    let min_value = 3000;
    let max_value = 4095;
    let normalized_value =
        (readout.saturating_sub(min_value)) as f32 / (max_value - min_value) as f32;
    // And now invert the value
    1.0 - normalized_value
}