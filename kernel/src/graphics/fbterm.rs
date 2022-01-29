use crate::{
    diag::terminal::Terminal,
    graphics::fb::{FbDisplay, GLOBAL_FB},
};
use alloc::string::{String, ToString};
use core::{
    default::Default,
    fmt::{Arguments, Write},
    iter::{repeat, FromIterator},
    ops::{Deref, DerefMut},
};
use embedded_graphics::text::{renderer::TextRenderer, Baseline};
use embedded_graphics_core::{
    geometry::Dimensions,
    pixelcolor::{Bgr888, Gray8, RgbColor},
    prelude::{DrawTarget, IntoStorage, Point, Size},
};
use fontdue::{layout::GlyphRasterConfig, Font, FontSettings, LineMetrics, Metrics};
use log::info;

const TAB_SIZE: i64 = 4;
const PADDING_PX: i64 = 3;
const CHAR_MARGIN: i64 = 2;
const FONT_SIZE: i64 = 15;

pub struct FbTextRender {
    font: Font,
    font_size: i64,
    current_pos: (i64, i64),
    line_metrics: LineMetrics,
    bg: Bgr888,
}

// TODO Support ANSI control codes

impl FbTextRender {
    pub fn new<T: Deref<Target = [u8]>>(font_data: T, bg: Bgr888) -> Self {
        let mut fb = GLOBAL_FB.lock();
        fb.fill(bg);
        fb.flush();
        let font = Font::from_bytes(
            font_data,
            FontSettings {
                scale: FONT_SIZE as _,
                ..Default::default()
            },
        )
        .expect("Failed to parse font");
        let line_metrics = font.horizontal_line_metrics(FONT_SIZE as _).unwrap();
        Self {
            font,
            font_size: FONT_SIZE,
            current_pos: (PADDING_PX, PADDING_PX),
            bg,
            line_metrics,
        }
    }

    pub fn write_fmt_colored(&mut self, args: Arguments<'_>, color: Bgr888) -> core::fmt::Result {
        let str = args.to_string();
        self.write_str_colored(&str, color)
    }

    pub fn write_str_colored(&mut self, string: &str, color: Bgr888) -> core::fmt::Result {
        for ch in string.chars() {
            self.write_char_colored_impl(ch, color)?
        }

        GLOBAL_FB.lock().flush();

        Ok(())
    }

    pub fn write_char_colored(&mut self, ch: char, color: Bgr888) -> core::fmt::Result {
        self.write_char_colored_impl(ch, color)
            .map(|_| GLOBAL_FB.lock().flush())
    }

    fn write_char_colored_impl(&mut self, ch: char, color: Bgr888) -> core::fmt::Result {
        if ch.is_control() {
            match ch {
                '\n' => self.advance_line(&mut **GLOBAL_FB.lock()),
                '\r' => self.current_pos.0 = 0,
                '\t' => {
                    self.write_str_colored(
                        &String::from_iter(repeat(' ').take(TAB_SIZE as usize)),
                        color,
                    )?;
                }
                _ => {}
            }
        } else {
            self.draw_char(ch, color)
        }
        Ok(())
    }

    fn alloc_place_for_char(&mut self, fb: &mut FbDisplay, width: i64) {
        let res = fb.mode.resolution();
        if self.current_pos.0 + width + PADDING_PX >= res.0 as i64 {
            self.advance_line(fb);
        }
    }

    fn advance_line(&mut self, fb: &mut FbDisplay) {
        let res = fb.mode.resolution();
        let line_size = self.line_metrics.new_line_size as u32 as i64 + 1;
        if self.current_pos.1 + line_size >= res.1 as i64 {
            let amount = self.current_pos.1 + line_size - (res.1 as i64) + 1;
            fb.scroll_up(amount as _, self.bg);
            self.current_pos.1 = res.1 as i64 - line_size - 1;
        } else {
            self.current_pos.1 += line_size;
        }
        self.current_pos.0 = PADDING_PX;
    }

    fn draw_char(&mut self, ch: char, color: Bgr888) {
        let (met, bmp) = self.font.rasterize(ch, self.font_size as u32 as f32);
        let mut fb = GLOBAL_FB.lock();
        self.alloc_place_for_char(&mut **fb, met.width as i64 + met.xmin as i64);
        for i in 0..met.height {
            for j in 0..met.width {
                let stride = fb.mode.stride();
                let fb_idx = (self.current_pos.0 + j as i64 + met.xmin as i64) as usize
                    + (self.current_pos.1
                        + i as i64
                        + self.line_metrics.ascent as i32 as i64
                        + self.line_metrics.line_gap as i32 as i64
                        - met.ymin as i64
                        - met.height as i64) as usize
                        * stride;
                let bmp_idx = i * met.width + j;

                let new_r = (color.r() as u64 * bmp[bmp_idx] as u64) / u8::MAX as u64;
                let new_g = (color.g() as u64 * bmp[bmp_idx] as u64) / u8::MAX as u64;
                let new_b = (color.b() as u64 * bmp[bmp_idx] as u64) / u8::MAX as u64;

                fb.write(fb_idx, Bgr888::new(new_r as _, new_g as _, new_b as _));
            }
        }

        self.current_pos.0 += met.advance_width as i32 as i64;
    }
}

impl Write for FbTextRender {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str_colored(s, Bgr888::WHITE)
    }

    fn write_char(&mut self, ch: char) -> core::fmt::Result {
        self.write_char_colored(ch, Bgr888::WHITE)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> core::fmt::Result {
        self.write_fmt_colored(args, Bgr888::WHITE)
    }
}
