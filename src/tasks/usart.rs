use embassy_executor::Spawner;

use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, BufferedUart, Config},
};
use embedded_io_async::{Read, Write};

use crate::Usart1Resources;

bind_interrupts!(struct Irqs {
    USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
});

#[embassy_executor::task]
pub async fn usart1_task(_spawner: Spawner, r: Usart1Resources) {
    let mut config = Config::default();
    config.baudrate = 115200;
    let mut tx_buf = [0u8; 256];
    let mut rx_buf = [0u8; 256];
    let mut usart = BufferedUart::new(
        r.usart,
        Irqs,
        r.rx_pin,
        r.tx_pin,
        &mut tx_buf,
        &mut rx_buf,
        config,
    )
    .unwrap();

    usart.write_all(b"[ Caw FOC ]\r\n").await.unwrap();

    let mut buf = [0; 1];
    loop {
        usart.read_exact(&mut buf[..]).await.unwrap();
        usart.write_all(&buf[..]).await.unwrap();
    }
}
