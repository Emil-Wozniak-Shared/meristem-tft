#![no_std]
#![allow(unused_imports)]
#![no_main]

use alloc::format;
use defmt_rtt as _;
use esp_backtrace as _;
extern crate alloc;

use alloc::string::ToString;
use defmt::info;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, Level, Output, Pull};
use esp_hal::peripherals::{Peripherals, ADC1};
use esp_hal::rng::Rng;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{init, main};
use esp_hal::analog::adc::{Adc, AdcCalBasic, AdcCalSource, AdcConfig, Attenuation};
use loadcell::LoadCell;
use meristem_tft::hw390::hw390::Hw390;
use meristem_tft::hx711::hx711::Loadcell;
use meristem_tft::tft::tft::TFT;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals: Peripherals = init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    ).unwrap();

    let delay = Delay::new();
    // pins
    let hw_390_pin = peripherals.GPIO0;
    let hx711_dt = Input::new(peripherals.GPIO5, Pull::None);
    let hx711_sck = Output::new(peripherals.GPIO6, Level::Low);
    // let mut backlight = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let rst = peripherals.GPIO8;
    let dc = peripherals.GPIO10;
    let tcs = peripherals.GPIO15;
    let mosi = peripherals.GPIO18; // sdo -> MOSI
    let sclk = peripherals.GPIO19;
    let miso = peripherals.GPIO20; // sdi -> MISO
    let cs = peripherals.GPIO23;

    let mut load_cell = Loadcell::new(hx711_sck, hx711_dt, delay);
    let mut adc1_config = AdcConfig::new();

    let mut pin = adc1_config.enable_pin_with_cal(hw_390_pin, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);
    let mut hw390 = Hw390::new(adc1, pin, );
    let mut buffer: [u8; 512] = [0_u8; 512];
    let mut tft = TFT::new(peripherals.SPI2, sclk, miso, mosi, cs, rst, dc,tcs, &mut buffer);
    // tft.clear(Rgb565::WHITE);
    tft.draw_smiley();
    info!("Device full started!");
    loop {
        let weight = load_cell.read_scaled();
        tft.println(weight.to_string().as_str(), 20, 20);
        let hw390_value = format!("{:.1$}", hw390.read(), 2);
        tft.println(hw390_value.as_str(), 20, 40);
        delay.delay_millis(500);
    }
}
