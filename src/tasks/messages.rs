use defmt::Format;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, signal::Signal,
};

#[derive(PartialEq, Debug, Format)]
pub enum Events {}

#[derive(PartialEq, Debug, Format)]
pub enum Commands {
    UsartTxBuf(&'static [u8]),
}

pub static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, Events, 10> = Channel::new();

pub static USART_WRITE_SIGNAL: Signal<CriticalSectionRawMutex, Commands> = Signal::new();
