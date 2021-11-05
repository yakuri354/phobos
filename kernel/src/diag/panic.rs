use core::panic::PanicInfo;
use log::error;

/// The global panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}
