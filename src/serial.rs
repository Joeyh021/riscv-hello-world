use core::convert::Infallible;
use embedded_hal::serial::{blocking::Write, ErrorType};
use volatile_register::*;

const SR_RX_FIFO_VALID_DATA: u32 = 1 << 0;
const SR_RX_FIFO_FULL: u32 = 1 << 1;
const SR_TX_FIFO_EMPTY: u32 = 1 << 2;
const SR_TX_FIFO_FULL: u32 = 1 << 3;
const UART_ADDR: usize = 0x60010000;

pub struct Serial(&'static mut UartRegs);

#[repr(C)]
pub struct UartRegs {
    rx_fifo: RO<u32>,
    tx_fifo: WO<u32>,
    status: RO<u32>,
    control: RW<u32>,
}

impl ErrorType for Serial {
    type Error = Infallible;
}

impl Write for Serial {
    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        for b in buffer {
            self.write_byte(*b)
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while (self.0.status.read() & SR_TX_FIFO_FULL) != 0 {}
        Ok(())
    }
}

impl Serial {
    pub fn new() -> Self {
        Serial(unsafe { (UART_ADDR as *mut UartRegs).as_mut().unwrap() })
    }

    pub fn write_byte(&mut self, byte: u8) {
        #[allow(clippy::while_immutable_condition)]
        while (self.0.status.read() & SR_TX_FIFO_FULL) != 0 {}
        unsafe { self.0.tx_fifo.write(byte as u32) }
    }

    pub fn println(&mut self, msg: &str) {
        self.print(msg);
        self.print("\r\n");
    }
    pub fn print(&mut self, msg: &str) {
        self.write(msg.as_bytes()).unwrap();
    }
}
