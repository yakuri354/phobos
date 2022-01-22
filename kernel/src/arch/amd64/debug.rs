use lazy_static::lazy_static;

use spin::Mutex as Spinlock;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Spinlock<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Spinlock::new(serial_port)
    };
}
