use core::{fmt::Debug, ops::DerefMut, ptr::NonNull};

use embedded_graphics_core::pixelcolor::{Rgb888, RgbColor};
use fontdue::{Font, FontSettings};
use log::LevelFilter;
use uefi::proto::console::gop::ModeInfo;

use crate::{
    diag::logger::GLOBAL_LOGGER,
    graphics::{
        fb::{FbDisplay, GLOBAL_FB},
        fbterm::FbTextRender,
        font::FIRA_CODE,
    },
};

pub mod logger;
pub mod panic;
pub mod terminal;

pub fn init() {
    if let Ok(()) = log::set_logger(&GLOBAL_LOGGER) {
        log::set_max_level(LevelFilter::Debug)
    }
}

pub fn reinit_with_fb(addr: NonNull<u8>, mode: ModeInfo) {
    {
        let mut fb = GLOBAL_FB.lock();
        fb.deref_mut().init(FbDisplay::new(addr.cast(), mode));
    }

    GLOBAL_LOGGER
        .lock()
        .reinit_with_fbterm(FbTextRender::new(FIRA_CODE, Rgb888::BLACK));
}
