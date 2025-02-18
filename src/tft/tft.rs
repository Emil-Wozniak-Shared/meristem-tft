#![allow(unused_imports)]

use alloc::string::ToString;
use core::convert::Infallible;
use embedded_graphics::mono_font::ascii::FONT_8X13;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle, Triangle};
use embedded_graphics::text::{Alignment, Text};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*
    ,
};
use embedded_hal_bus::spi::{DeviceError, ExclusiveDevice, NoDelay};
use esp_hal::delay::Delay;
use esp_hal::gpio::{GpioPin, Level, Output};
use esp_hal::peripherals::SPI2;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::{Error, Mode};
use esp_hal::time::RateExtU32;
use esp_hal::Blocking;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ILI9341Rgb565;
use mipidsi::{Builder, Display};

type TFTSpiDevice<'spi> = ExclusiveDevice<Spi<'spi, Blocking>, Output<'spi>, NoDelay>;
type TFTSpiInterface<'spi> = SpiInterface<'spi, TFTSpiDevice<'spi>, Output<'spi>>;

pub struct TFT<'spi> {
    display:
        Display<
            TFTSpiInterface<'spi>,
            ILI9341Rgb565,
            Output<'spi>
        >,
}

impl<'spi> TFT<'spi> {
    pub fn new(
        spi2: SPI2,
        sclk: GpioPin<19>,
        miso: GpioPin<20>,
        mosi: GpioPin<18>,
        cs: GpioPin<23>,
        rst: GpioPin<4>,
        dc: GpioPin<2>,
        tcs: GpioPin<15>,
        buffer: &'spi mut [u8],
    ) -> TFT<'spi> {
        let rst_output = Output::new(rst, Level::Low);
        let dc_output = Output::new(dc, Level::Low);
        let spi = Spi::new(spi2,Self::create_config())
            .unwrap()
            .with_sck(sclk)
            .with_miso(miso) // order matters
            .with_mosi(mosi) // order matters
            // .with_cs(cs)
            ;
        let cs_output = Output::new(cs, Level::High);
        let spi_device: ExclusiveDevice<Spi<Blocking>, Output, NoDelay> = ExclusiveDevice::new_no_delay(spi, cs_output).unwrap();

        let di: SpiInterface<ExclusiveDevice<Spi<Blocking>, Output, NoDelay>, Output> = SpiInterface::new(spi_device, dc_output, buffer);
        let display = Builder::new(ILI9341Rgb565, di)
            .reset_pin(rst_output)
            .init(&mut Delay::new())
            .unwrap();

        TFT { display }
    }

    fn create_config() -> Config {
        Config::default()
            .with_frequency(100.kHz())
            .with_mode(Mode::_0)
    }

    pub fn clear(&mut self, color: Rgb565) {
        self.display.clear(color).unwrap();
    }

    pub fn get_touch(&mut self) {
        // match self.xpt2046.get() {
        //     Ok(x) => {
        //         self.println(x.1.to_string().as_str(), 30, 50);
        //         self.println(x.0.to_string().as_str(), 30, 40);
        //     }
        //     Err(e) => {
        //         self.println(e.to_string().as_str(), 30, 40);
        //     }
        // }
    }

    pub fn println(&mut self, text: &str, x: i32, y: i32) {
        let style = MonoTextStyle::new(&FONT_8X13, Rgb565::RED);
        // refresh block
        Rectangle::new(Point::new(x - 20, y - 20), Size::new(240, 30))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
            .draw(&mut self.display).unwrap();
        //draw new text
        Text::with_alignment(
            text,
            Point::new(x, y),
            style,
            Alignment::Center,
        ).draw(&mut self.display).unwrap();
    }

    pub fn draw_smiley(&mut self) {
        Circle::new(Point::new(50, 100), 40)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
            .draw(&mut self.display).unwrap();

        Circle::new(Point::new(50, 200), 40)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
            .draw(&mut self.display).unwrap();

        Triangle::new(
            Point::new(130, 140),
            Point::new(130, 200),
            Point::new(160, 170),
        )
            .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
            .draw(&mut self.display).unwrap();

        Triangle::new(
            Point::new(130, 150),
            Point::new(130, 190),
            Point::new(150, 170),
        )
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut self.display).unwrap();
    }
}
