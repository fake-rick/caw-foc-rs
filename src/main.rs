#![no_std]
#![no_main]

mod comm;
mod hws;
mod macros;
mod resources;
mod tasks;

use defmt::*;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    time::Hertz,
};
use embassy_time::Timer;
use hws::drv8323rs::DRV8232RS;
use resources::*;
use tasks::{
    can::{can2_task, can3_task},
    drv8323::drv8323_task,
    messages::{Commands, USART_WRITE_SIGNAL},
    state::check_state_task,
    usart::usart1_task,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config: embassy_stm32::Config = Default::default();
    {
        use embassy_stm32::rcc::*;
        // 配置RCC时钟树
        config.rcc.hsi = false;
        config.rcc.hse = Some(Hse {
            freq: Hertz(16_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL80,
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV2), // FD CAN时钟
            divr: Some(PllRDiv::DIV2), // 系统时钟
        });
        config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV1;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
    }

    let p = embassy_stm32::init(config);
    let r = split_resources!(p);

    let drv_spi = init_spi3(r.spi3).await;
    let drv_nss = Output::new(p.PA15, Level::High, Speed::Low);
    let drv_spi_dev = SpiDevice::new(drv_spi, drv_nss);
    info!("[ CawFOC ]");

    // can bus configure
    let mut can_stb = Output::new(p.PD2, Level::High, Speed::High);
    can_stb.set_low();

    spawner.spawn(can2_task(spawner, r.can2)).unwrap();
    spawner.spawn(can3_task(spawner, r.can3)).unwrap();
    spawner.spawn(usart1_task(spawner, r.usart1)).unwrap();
    spawner.spawn(check_state_task(spawner, r.state)).unwrap();
    spawner
        .spawn(drv8323_task(spawner, DRV8232RS::new(drv_spi_dev).await))
        .unwrap();
    loop {
        Timer::after_millis(1000).await;
    }
}
