use crate::{arch::debug::*, diag::logger::FB_LOGGER};
use log::LevelFilter;

pub mod logger;
pub mod panic;

pub fn init() {
    log::set_logger(&FB_LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
}
