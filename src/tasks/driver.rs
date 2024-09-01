use defmt::*;
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::peripherals::TIM1;
use embassy_stm32::time::khz;
#[cfg(feature = "pwm6x")]
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    mode::Async,
    spi::Spi,
};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;

use crate::resources::*;

use crate::{hws::drv8323rs::*, Drv8323Resources};

#[cfg(feature = "pwm3x")]
async fn create_pwm(r: Timer1Resources) -> SimplePwm<'static, TIM1> {
    use embassy_stm32::timer::low_level::CountingMode;

    let ch1 = PwmPin::new_ch1(r.tim1_ch1, OutputType::PushPull);
    let ch2 = PwmPin::new_ch2(r.tim1_ch2, OutputType::PushPull);
    let ch3 = PwmPin::new_ch3(r.tim1_ch3, OutputType::PushPull);

    let mut ch1n = Output::new(r.tim1_ch1n, Level::High, Speed::Low);
    let mut ch2n = Output::new(r.tim1_ch2n, Level::High, Speed::Low);
    let mut ch3n = Output::new(r.tim1_ch3n, Level::High, Speed::Low);
    ch1n.set_high();
    ch2n.set_high();
    ch3n.set_high();

    SimplePwm::new(
        r.tim1,
        Some(ch1),
        Some(ch2),
        Some(ch3),
        None,
        khz(100),
        CountingMode::CenterAlignedBothInterrupts,
    )
}

#[cfg(feature = "pwm6x")]
async fn create_pwm(r: Timer1Resources) -> ComplementaryPwm<'static, TIM1> {
    use embassy_stm32::timer::low_level::CountingMode;

    let ch1 = PwmPin::new_ch1(r.tim1_ch1, OutputType::PushPull);
    let ch2 = PwmPin::new_ch2(r.tim1_ch2, OutputType::PushPull);
    let ch3 = PwmPin::new_ch3(r.tim1_ch3, OutputType::PushPull);

    let ch1n = ComplementaryPwmPin::new_ch1(r.tim1_ch1n, OutputType::PushPull);
    let ch2n = ComplementaryPwmPin::new_ch2(r.tim1_ch2n, OutputType::PushPull);
    let ch3n = ComplementaryPwmPin::new_ch3(r.tim1_ch3n, OutputType::PushPull);

    ComplementaryPwm::new(
        r.tim1,
        Some(ch1),
        Some(ch1n),
        Some(ch2),
        Some(ch2n),
        Some(ch3),
        Some(ch3n),
        None,
        None,
        khz(100),
        CountingMode::CenterAlignedBothInterrupts,
    )
}

#[embassy_executor::task]
pub async fn driver_task(
    _spawner: Spawner,
    mut drv: DRV8232RS<SpiDevice<'static, NoopRawMutex, Spi<'static, Async>, Output<'static>>>,
    t: Timer1Resources,
    r: Drv8323Resources,
) {
    #[cfg(feature = "pwm3x")]
    info!("pwm mode 3x");
    #[cfg(feature = "pwm6x")]
    info!("pwm mode 6x");
    let mut pwm = create_pwm(t).await;
    let max = pwm.get_max_duty();
    info!("Max duty: {}", max);

    pwm.set_duty(Channel::Ch1, 0);
    pwm.set_duty(Channel::Ch2, 0);
    pwm.set_duty(Channel::Ch3, 0);
    pwm.enable(Channel::Ch1);
    pwm.enable(Channel::Ch2);
    pwm.enable(Channel::Ch3);

    Timer::after_micros(100).await;
    let mut enable = Output::new(r.enable, Level::Low, Speed::Low);
    let mut cal = Output::new(r.cal, Level::High, Speed::Low);
    Timer::after_micros(100).await;
    enable.set_high();
    cal.set_low();
    Timer::after_micros(100).await;
    drv.calibrate().await;
    Timer::after_micros(100).await;
    #[cfg(feature = "pwm3x")]
    drv.write_dcr(0x0, DIS_GDF_DIS, 0x0, PWM_MODE_3X, 0x0, 0x0, 0x0, 0x0, 0x1)
        .await;
    #[cfg(feature = "pwm6x")]
    drv.write_dcr(0x0, DIS_GDF_DIS, 0x0, PWM_MODE_6X, 0x0, 0x0, 0x0, 0x0, 0x1)
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
    Timer::after_micros(100).await;
    drv.dbg_reg_val().await;
    drv.enable_gd().await;

    pwm.set_duty(Channel::Ch1, max / 2);
    pwm.set_duty(Channel::Ch2, max / 2);
    pwm.set_duty(Channel::Ch3, max / 2);

    loop {
        Timer::after_millis(3000).await;
    }
}
