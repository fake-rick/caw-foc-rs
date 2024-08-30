use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::{gpio::Output, mode::Async, spi::Spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;

use crate::hws::drv8323rs::DRV8232RS;

#[embassy_executor::task]
pub async fn drv8323_task(
    _spawner: Spawner,
    drv: DRV8232RS<SpiDevice<'static, NoopRawMutex, Spi<'static, Async>, Output<'static>>>,
) {
    loop {
        Timer::after_millis(1000).await;
    }
}
