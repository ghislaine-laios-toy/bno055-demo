use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::FromValueType;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take()?;

    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let baudrate = 100u32.kHz();
    let config = I2cConfig::new().baudrate(baudrate.into());

    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;
}
