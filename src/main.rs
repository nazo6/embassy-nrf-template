#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::Output,
    usb::{Driver, vbus_detect::HardwareVbusDetect},
};

bind_interrupts!(struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
    CLOCK_POWER => embassy_nrf::usb::vbus_detect::InterruptHandler;
    // SAADC => embassy_nrf::saadc::InterruptHandler;
    // TWISPI0 => embassy_nrf::twim::InterruptHandler<embassy_nrf::peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    spawner.spawn(logger_task(Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs))).unwrap());

    let mut output = Output::new(
        p.P0_22,
        embassy_nrf::gpio::Level::Low,
        embassy_nrf::gpio::OutputDrive::Standard,
    );

    loop {
        output.set_high();
        embassy_time::Timer::after_millis(500).await;
        output.set_low();
        embassy_time::Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, HardwareVbusDetect>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
