use crate::{
    arch::debug::SERIAL1,
    graphics::{fb::GLOBAL_FB, font::FIRA_CODE},
    sync::irq_lock::IRQLocked,
};
use alloc::string::String;
use core::{cmp::min, fmt::Write};
use embedded_font::FontTextStyleBuilder;
use embedded_graphics::text::{renderer::TextRenderer, Baseline};
use embedded_graphics_core::{
    geometry::Point,
    pixelcolor::{Bgr888, RgbColor, WebColors},
};
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use rusttype::Font;

pub static FB_LOGGER: IRQLocked<FbLogger> = IRQLocked::new(FbLogger::new());

const LINE_SPACING: u32 = 5;
const FONT_SIZE: u32 = 20;
const ROW_HEIGHT: u32 = LINE_SPACING + FONT_SIZE;

pub struct FbLogger {
    early: bool,
    curr_line: u32,
    rows: u32,
}

impl Log for IRQLocked<FbLogger> {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // TODO Move into display
        let mut state = self.lock();

        SERIAL1
            .lock()
            .write_fmt(format_args!(
                "[{}] {}\n",
                record.level().as_str().chars().nth(0).unwrap(),
                record.args()
            ))
            .unwrap();

        if state.early {
            return;
        }

        let mut str = String::new();
        str
            .write_fmt(format_args!(
                "[{}] {}\n",
                record.level().as_str().chars().nth(0).unwrap(),
                record.args()
            ))
            .expect("Writing log message failed");
        let fts = FontTextStyleBuilder::new(
            Font::try_from_bytes(FIRA_CODE).expect("Could not load Fira Code font"),
        )
        .text_color(match record.level() {
            Level::Error => Bgr888::CSS_TOMATO,
            Level::Warn => Bgr888::CSS_LIGHT_SALMON,
            Level::Info => Bgr888::WHITE,
            Level::Debug => Bgr888::CSS_ANTIQUE_WHITE,
            Level::Trace => Bgr888::CSS_AZURE,
        })
        .background_color(Bgr888::BLACK)
        .font_size(FONT_SIZE)
        .build();

        str.retain(|x| !x.is_control());

        let mut fb = GLOBAL_FB.lock();

        // FIXME This assumes a monospace font
        // Also, the measurement is really rough

        let char_w = fts
            .measure_string("aa", Point::new(0, 0), Baseline::Top)
            .bounding_box
            .size
            .width
            / 2;
        let mut max_chars = fb.mode.resolution().1 as u32 / char_w;

        assert!(max_chars > 0);

        max_chars -= 1;

        let lines: u32 = if str.len() as u32 % max_chars == 0 {
            str.len() as u32 / max_chars
        } else {
            str.len() as u32 / max_chars + 1
        };

        let mut point = (char_w / 2, ROW_HEIGHT * state.curr_line);

        if state.curr_line == state.rows - 1 {
            fb.scroll_up((lines * ROW_HEIGHT) as usize, Bgr888::BLACK);
            point.1 = ROW_HEIGHT * (state.rows - lines);
        } else {
            state.curr_line += 1;
        }

        point.1 += LINE_SPACING;

        for i in 0..lines {
            point.1 += i * ROW_HEIGHT;
            fts.draw_string(
                &str[((i * max_chars) as usize)
                    ..min((i as usize + 1) * max_chars as usize, str.len())],
                Point::new(point.0 as _, point.1 as _),
                Baseline::Top,
                &mut **fb,
            );
        }

        fb.flush();
    }

    fn flush(&self) {
        // TODO
    }
}

impl FbLogger {
    pub const fn new() -> Self {
        Self {
            early: true,
            curr_line: 0,
            rows: 0,
        }
    }

    pub fn reinit_with_fb(&mut self) {
        let (_, h) = GLOBAL_FB.lock().mode.resolution();
        self.early = false;
        self.rows = h as u32 / ROW_HEIGHT;
    }
}
