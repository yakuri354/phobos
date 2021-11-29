use crate::arch::debug::*;

pub mod panic;

pub fn init() {
    init_debug_logger().unwrap()
}
