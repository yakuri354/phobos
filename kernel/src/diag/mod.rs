use crate::{
    diag::logger::GLOBAL_LOGGER,
    graphics::{
        fb::{FbDisplay, GLOBAL_FB},
        fbterm::FbTextRender,
        font::FIRA_CODE,
    },
};
use core::{ops::DerefMut, ptr::NonNull};
use core::fmt::Debug;
use embedded_graphics_core::pixelcolor::{Bgr888, RgbColor};
use fontdue::{Font, FontSettings};
use log::LevelFilter;
use uefi::proto::console::gop::ModeInfo;

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
        .reinit_with_fbterm(FbTextRender::new(FIRA_CODE, Bgr888::BLACK));
}
