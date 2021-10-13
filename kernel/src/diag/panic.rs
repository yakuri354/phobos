use core::panic::PanicInfo;

/// The global panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO
    loop {}
}
