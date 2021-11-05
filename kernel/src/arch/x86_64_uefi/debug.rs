use lazy_static::lazy_static;
use crate::sync::spin::Spinlock;
use uart_16550::SerialPort;
use core::fmt::Write;
use log::{Metadata, Record, SetLoggerError, LevelFilter};

lazy_static! {
    static ref SERIAL1: Spinlock<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Spinlock::new(serial_port)
    };
}

static DBG_LOGGER: SerialLogger = SerialLogger {};

pub fn init_debug_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&DBG_LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}

struct SerialLogger;

impl log::Log for SerialLogger {
    fn enabled(&self, _m: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        SERIAL1.lock().write_fmt(
            format_args!(
                "{}: {}",
                record.level(),
                record.args()
            ),
            // format_args!(
            //     "{:>8}: {} ({}, {}:{})\n",
            //     record.level(),
            //     record.args(),
            //     record.target(),
            //     record.file().unwrap_or("<unknown>"),
            //     record.line().unwrap_or(0),
            // ),
        );
    }

    fn flush(&self) {}
}