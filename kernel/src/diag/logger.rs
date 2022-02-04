use core::fmt::Write;

use embedded_graphics_core::pixelcolor::{Rgb888, RgbColor, WebColors};
use log::{Level, Log, Metadata, Record};

use crate::{
    arch::debug::SERIAL1,
    data::late_init::LateInit,
    graphics::{fb::FbDisplay, fbterm::FbTextRender},
    sync::irq_lock::IRQLocked,
};

pub static GLOBAL_LOGGER: IRQLocked<DefaultLogger> = IRQLocked::new(DefaultLogger::new());

pub struct DefaultLogger {
    term: Option<FbTextRender>,
}

// TODO Buffering

impl Log for IRQLocked<DefaultLogger> {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if let Some(mut serial) = SERIAL1.try_lock() {
            serial
                .write_fmt(format_args!(
                    "[{}] {}\n",
                    record.level().as_str().chars().next().unwrap(),
                    record.args()
                ))
                .expect("Could not write log message to serial port");
        }

        if self.is_locked() {
            return;
        }

        if let Some(term) = self.lock().term.as_mut() {
            term.write_fmt_colored(
                format_args!(
                    "[{}] {}\n",
                    record.level().as_str().chars().next().unwrap(),
                    record.args()
                ),
                match record.level() {
                    Level::Error => Rgb888::CSS_TOMATO,
                    Level::Warn => Rgb888::CSS_LIGHT_SALMON,
                    Level::Info => Rgb888::WHITE,
                    Level::Debug => Rgb888::CSS_ANTIQUE_WHITE,
                    Level::Trace => Rgb888::CSS_AZURE,
                },
            )
            .expect("Could not write log record to framebuffer")
        }
    }

    fn flush(&self) {
        // TODO
    }
}

impl DefaultLogger {
    pub const fn new() -> Self {
        Self { term: None }
    }

    pub fn reinit_with_fbterm(&mut self, term: FbTextRender) {
        self.term = Some(term)
    }
}
