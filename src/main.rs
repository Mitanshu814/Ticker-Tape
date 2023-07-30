use anyhow::{Result};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

mod led;
use led::{RGB8, WS2812RMT};

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // The constant CONFIG is auto-generated by toml_config
    let app_config = CONFIG;
    let mut led = WS2812RMT::new(peripherals.pins.gpio2, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(50, 50, 0))?;

    info!("Hello, world! {:?}{:?}", app_config.wifi_ssid, app_config.wifi_psk);

    loop {
        // Blue!
        led.set_pixel(RGB8::new(0, 0, 50))?;
        // Wait...
        std::thread::sleep(std::time::Duration::from_secs(1));
        info!("Hello, world!");

        // Green!
        led.set_pixel(RGB8::new(0, 50, 0))?;
        // Wait...
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
