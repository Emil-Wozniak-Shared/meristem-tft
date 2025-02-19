use defmt::info;
use embedded_hal::delay::DelayNs;
use esp_hal::analog::adc::{Adc, AdcCalSource, AdcConfig, AdcPin, Attenuation};
use esp_hal::delay::Delay;
use esp_hal::gpio::GpioPin;
use esp_hal::peripherals::ADC1;
use nb::block;

// https://github.com/plantineers/edge/blob/main/src/main.rs
/// A struct containing the hardware(via pointers to the right addresses of course) for the hw390 moisture sensor
pub struct Hw390<'a> {
    adc: Adc<'a, ADC1>,
    pin: AdcPin<GpioPin<0>, ADC1>,
    delay: Delay
}

impl<'a> Hw390<'a> {
    pub fn new(adc: Adc<'a, ADC1>, pin: AdcPin<GpioPin<0>, ADC1>) -> Self {
        let delay = Delay::new();
        Hw390 { adc, pin, delay }
    }

    /// Read out moisture data from the hw390 sensor and normalise it
    /// The hw-390 moisture sensor returns a value between 3000 and 4095
    // From our measurements the sensor was in water at 3000 and in air at 4095
    // We want to normalise the values to be between 0 and 1, so that 1 is in water and 0 is in air
    pub fn read(&mut self) -> f32 {
        let min_value = 1;
        let max_value = 4810;

        self.delay.delay_ms(100);
        let calibrate = AdcConfig::<ADC1>::adc_calibrate(Attenuation::_11dB, AdcCalSource::Ref);
        let readout = block!(self.adc.read_oneshot(&mut self.pin)).unwrap();
        self.delay.delay_ms(100);
        let normalized_value = readout.saturating_sub(min_value) as f32 / (max_value  as f32 - min_value as f32);
        let value = ((readout as f32) * 3.3) / (max_value as f32);// (value * 3.3) / max_value;
        info!("HW-390 readout: {} value: {} calibrate {} normal {}", readout, value, calibrate, normalized_value);
        value
    }
}


