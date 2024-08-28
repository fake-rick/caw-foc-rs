use crate::assign_resources;
use embassy_stm32::peripherals;

assign_resources! {
    state: StateResources {
        drv_sta_pin: PB1,
        mcu_sta_pin: PB2,
    },
    can2: Can2Resources {
        fdcan: FDCAN2,
        rx_pin: PB5,
        tx_pin: PB6,
    },
    can3: Can3Resources {
        fdcan: FDCAN3,
        rx_pin: PB3,
        tx_pin: PB4,
    }
    usart1: Usart1Resources {
        usart: USART1,
        rx_pin: PC5,
        tx_pin: PC4,
    }
}
