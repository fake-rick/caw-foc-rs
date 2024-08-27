use core::cell::RefCell;

use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, BufferedUart, Uart},
};
use embassy_sync::blocking_mutex::NoopMutex;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
});
