use crate::{
    data::late_init::LateInit,
    graphics::{fb::GLOBAL_FB, font::FIRA_CODE},
    sync::irq_lock::IRQLocked,
};
use alloc::string::String;
use core::{
    cmp::min,
    fmt::Write,
    ops::{Deref, DerefMut},
};
use embedded_font::{FontTextStyle, FontTextStyleBuilder};
use embedded_graphics::text::{renderer::TextRenderer, Baseline};
use embedded_graphics_core::{
    geometry::Dimensions,
    pixelcolor::{Bgr888, IntoStorage},
    prelude::{Point, RgbColor, WebColors},
};
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use rusttype::Font;
use spin::Mutex as Spinlock;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Spinlock<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Spinlock::new(serial_port)
    };
}
