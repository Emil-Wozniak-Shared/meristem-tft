#![no_std]
#![no_main]

extern crate alloc;
use defmt::info;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};

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
use mipidsi::models::ILI9486Rgb565;
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
    let dma_channel = peripherals.DMA_CH0;
    // let mut delay = Ets;
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);
    let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    let mut spi = Spi::new(
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

    // let spi_interface = SPIInterface::new(spi, dc);

    // let mut lcd = Ili9341::new(
    //     spi_interface,
    //     rst,
    //     &mut Delay::new(),
    //     PortraitFlipped,
    //     DisplaySize240x320,
    // ).unwrap();
    let mut buffer = [0_u8; 512];
    let cs_output = Output::new(cs, Level::High);
    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs_output).unwrap();
    let di = SpiInterface::new(spi_device, dc, &mut buffer);
    let mut display = Builder::new(ILI9486Rgb565, di)
        .reset_pin(rst)
        .init(&mut Delay::new())
        .unwrap();


    // Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::with_alignment(
        "First line\nSecond line",
        Point::new(20, 30),
        style,
        Alignment::Center,
    ).draw(&mut display).unwrap();

    loop {
        info!("Hello world!");
        delay.delay_millis(500);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.23.1/examples/src/bin
}
