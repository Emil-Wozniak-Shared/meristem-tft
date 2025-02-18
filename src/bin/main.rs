
#![no_std]
#![allow(unused_imports)]
#![no_main]

extern crate alloc;

use defmt::export::display;
use defmt_rtt as _;
use esp_backtrace as _;
use defmt::info;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Triangle};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{Level, Output};
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::Mode;
use esp_hal::time::RateExtU32;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{dma_buffers, main};
use mipidsi::interface::SpiInterface;
use mipidsi::models::{ILI9341Rgb565, ILI9486Rgb565};
use mipidsi::Builder;

#[main]
fn main() -> ! {
    // generator version: 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
        .unwrap();

    let delay = Delay::new();
    let rst =  Output::new(peripherals.GPIO4, Level::Low);
    let dc = Output::new(peripherals.GPIO2, Level::Low);
    // let mut backlight = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let sclk = peripherals.GPIO19;
    let mosi = peripherals.GPIO18; // sdo -> MOSI
    let miso = peripherals.GPIO20; // sdi -> MISO
    let cs = peripherals.GPIO23;

    // let mut delay = Ets;
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);


    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(100.kHz())
            .with_mode(Mode::_0),
    )
        .unwrap()
        .with_sck(sclk)
        .with_miso(miso) // order matters
        .with_mosi(mosi) // order matters
        // .with_cs(cs)
        ;
    let mut buffer = [0_u8; 512];
    let cs_output = Output::new(cs, Level::High);
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs_output).unwrap();
    let di = SpiInterface::new(spi_device, dc, &mut buffer);
    let mut display = Builder::new(ILI9341Rgb565, di)
        .reset_pin(rst)
        .init(&mut Delay::new())
        .unwrap();

    display.clear(Rgb565::WHITE).unwrap();
    // Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::with_alignment(
        "First line\nSecond line",
        Point::new(20, 30),
        style,
        Alignment::Center,
    ).draw(&mut display).unwrap();

    draw_smiley(&mut display).unwrap();
    loop {
        info!("Hello world!");
        Text::with_alignment(
            "generate(10)",
            Point::new(40, 60),
            style,
            Alignment::Center,
        ).draw(&mut display).unwrap();
        delay.delay_millis(500);
    }
}

fn draw_smiley<T: DrawTarget<Color = Rgb565>>(display: &mut T) -> Result<(), T::Error> {
    // Draw the left eye as a circle located at (50, 100), with a diameter of 40, filled with white
    Circle::new(Point::new(50, 100), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(display)?;

    // Draw the right eye as a circle located at (50, 200), with a diameter of 40, filled with white
    Circle::new(Point::new(50, 200), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(display)?;

    // Draw an upside down red triangle to represent a smiling mouth
    Triangle::new(
        Point::new(130, 140),
        Point::new(130, 200),
        Point::new(160, 170),
    )
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(display)?;

    // Cover the top part of the mouth with a black triangle so it looks closed instead of open
    Triangle::new(
        Point::new(130, 150),
        Point::new(130, 190),
        Point::new(150, 170),
    )
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(display)?;

    Ok(())
}