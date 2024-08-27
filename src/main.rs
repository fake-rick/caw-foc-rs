#![no_std]
#![no_main]

mod comm;
mod drivers;
mod macros;
mod resources;
mod tasks;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    can,
    gpio::{AnyPin, Level, Output, Pin, Speed},
    time::Hertz,
};
use embassy_time::Timer;
use resources::*;
use tasks::{
    can::{can2_task, can3_task},
    state::check_state_task,
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

    info!("[ CawFOC ]");

    // can bus configure
    let mut can_stb = Output::new(p.PD2, Level::High, Speed::High);
    can_stb.set_low();

    spawner.spawn(can2_task(spawner, r.can2)).unwrap();
    spawner.spawn(can3_task(spawner, r.can3)).unwrap();
    spawner.spawn(check_state_task(spawner, r.state)).unwrap();

    loop {
        Timer::after_millis(1000).await;
    }
}
