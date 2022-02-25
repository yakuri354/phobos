# Keyboard driver

Keyboard driver is implemented as an async stream of keycodes. The keycodes are added into a queue while handling PS/2 interrupt and later yielded from the stream.

Scancodes are enqueued here:
```rust
extern "x86-interrupt" fn keyboard(_frame: InterruptStackFrame) {
    let code: u8 = unsafe { Port::new(0x60).read() };
    crate::device::ps2kb::add_scancode(code);
    unsafe {
        PICs.lock()
            .notify_end_of_interrupt(IntIdx::Keyboard.as_u8());
    }
}
```

And later processed in async fashion:
```rust
while let Some(scancode) = scancodes.next().await {
    do_something_with(scancode);
}
```
Notice the keyword `await`.

The driver supports all keyboards, including those connected with USB and PS/2

#### Also see:
- [Async and multitasking in phobos](async.md)
- [PS/2 keyboard protocol](https://wiki.osdev.org/PS/2_Keyboard)