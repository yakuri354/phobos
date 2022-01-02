use crate::arch::debug::*;

pub mod panic;

#[macro_export]
macro_rules! bug_reached {
    () => {
        ::core::panic!(::core::format_args!("BUG detected on {}:{}; Statement should not be reachable"), ::core::file!(). ::core::line!())
    }
}

pub fn init() {
    init_debug_logger().unwrap()
}
