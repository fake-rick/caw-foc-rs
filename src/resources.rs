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
    },
    usart1: Usart1Resources {
        usart: USART1,
        rx_pin: PC5,
        tx_pin: PC4,
        dma1_ch2: DMA1_CH2,
        dma1_ch3: DMA1_CH3,
    },
    spi3: Spi3Resources {
        spi: SPI3,
        cs: PA15,
        mosi: PC12,
        miso: PC11,
        sck: PC10,
    },
}
