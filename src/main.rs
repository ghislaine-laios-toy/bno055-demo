use std::time::Duration;

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use esp_idf_svc::hal::delay::{Delay, FreeRtos};
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripheral::Peripheral;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::FromValueType;
use esp_idf_svc::timer::EspTaskTimerService;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let mut imu = {
        let sda = peripherals.pins.gpio21;
        let scl = peripherals.pins.gpio22;
        let reset_pin = peripherals.pins.gpio19;

        let i2c = {
            let baudrate = 20u32.kHz();
            let config = I2cConfig::new().baudrate(baudrate.into());

            I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap()
        };
        
        #[allow(unused_labels)]
        'reset_imu: {
            let mut reset_pin = PinDriver::output(reset_pin).unwrap();

            reset_pin.set_low().unwrap();

            FreeRtos.delay_ms(1);

            reset_pin.set_high().unwrap();

            FreeRtos.delay_ms(500);
        }

        let mut imu = bno055::Bno055::new(i2c);

        imu.init(&mut FreeRtos).unwrap();

        imu.set_mode(bno055::BNO055OperationMode::NDOF, &mut FreeRtos)
            .unwrap();

        imu
    };

    let timer = {
        let timer_service = EspTaskTimerService::new().unwrap();

        timer_service.timer(move || {
            let quat = imu.euler_angles().unwrap();

            log::info!("{:?}", quat);
        })
    }
    .unwrap();

    timer.every(Duration::from_millis(20)).unwrap();

    loop {
        FreeRtos::delay_ms(1000);
    }
}
