use defmt::info;
use esp_hal::analog::adc::{Adc, AdcPin};
use esp_hal::gpio::GpioPin;
use esp_hal::peripherals::ADC1;
use nb::block;

// https://github.com/plantineers/edge/blob/main/src/main.rs
/// A struct containing the hardware(via pointers to the right addresses of course) for the hw390 moisture sensor
pub struct Hw390<'a> {
    adc: Adc<'a, ADC1>,
    pin: AdcPin<GpioPin<2>, ADC1>,
}

impl<'a> Hw390<'a> {
    pub fn new(adc: Adc<'a, ADC1>, pin: AdcPin<GpioPin<2>, ADC1>) -> Self {
        Hw390 { adc, pin }
    }

    /// Read out moisture data from the hw390 sensor and normalise it
    pub fn read(&mut self) -> f32 {
        let mut collector = 0;
        for _ in 0..20 {
            collector += block!(self.adc.read_oneshot(&mut self.pin)).unwrap() as i32;
        }
        let readout = collector / 20;
        let value = normalise_humidity_data(readout); // as f32 * 3.3) / (4095.0);
        value
    }
}

/// The hw-390 moisture sensor returns a value between 3000 and 4095
/// From our measurements the sensor was in water at 3000 and in air at 4095
/// We want to normalise the values to be between 0 and 1, so that 1 is in water and 0 is in air
fn normalise_humidity_data(readout: i32) -> f32 {
    info!("HW-390 readout: {}", readout);
    let max_value = 4096.0;
    ((readout as f32) * 3.3) / (max_value)
}
