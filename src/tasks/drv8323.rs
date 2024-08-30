use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    mode::Async,
    spi::Spi,
};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;

use crate::{hws::drv8323rs::*, Drv8323Resources};

#[embassy_executor::task]
pub async fn drv8323_task(
    _spawner: Spawner,
    mut drv: DRV8232RS<SpiDevice<'static, NoopRawMutex, Spi<'static, Async>, Output<'static>>>,
    r: Drv8323Resources,
) {
    // configure drv8323
    Timer::after_micros(100).await;
    let mut enable = Output::new(r.enable, Level::Low, Speed::Low);
    let mut cal = Output::new(r.cal, Level::High, Speed::Low);
    Timer::after_micros(100).await;
    enable.set_high();
    cal.set_low();
    Timer::after_micros(100).await;
    drv.calibrate().await;
    Timer::after_micros(100).await;
    drv.write_dcr(0x0, DIS_GDF_DIS, 0x0, PWM_MODE_3X, 0x0, 0x0, 0x0, 0x0, 0x1)
        .await;
    Timer::after_micros(100).await;
    drv.write_csacr(0x0, 0x1, 0x0, CSA_GAIN_40, 0x0, 0x1, 0x1, 0x1, SEN_LVL_1_0)
        .await;
    Timer::after_micros(100).await;
    drv.write_csacr(0x0, 0x1, 0x0, CSA_GAIN_40, 0x1, 0x0, 0x0, 0x0, SEN_LVL_1_0)
        .await;
    Timer::after_micros(100).await;
    drv.write_ocpcr(
        TRETRY_50US,
        DEADTIME_50NS,
        OCP_NONE,
        OCP_DEG_8US,
        VDS_LVL_1_88,
    )
    .await;
    drv.disable_gd().await;
    loop {
        drv.print_faults().await;
        Timer::after_millis(1000).await;
    }
}
