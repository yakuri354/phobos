# UEFI Bootloader

phobos uses a custom UEFI bootloader which loads the kernel, sets up the necessary environment and passes initialization data into the kernel through a KernelArgs struct.

The initialization procedure:
1. Initialize GOP framebuffer, set appropriate mode
2. Map all physical memory at `PHYS_MAP_OFFSET`
3. Map the framebuffer
4. Call `SetVirtualAddressMap()`
5. Load kernel binary
6. Call `ExitBootServices()`
7. Switch to a new stack
8. Jump to the kernel entry point

```rust,ignore
#[repr(C)]
pub struct KernelArgs {
    pub mmap: ArrayVec<MemoryDescriptor, 512>,
    pub uefi_rst: SystemTable<Runtime>,
    pub fb_addr: *mut u8,
    pub fb_info: ModeInfo,
}
```

Bootloader code is located in `kernel/arch/amd64/boot`.
Crate `boot_lib` provides common structures and constants for kernel and bootloader.

#### Also see:
- [UEFI - OSDev Wiki](https://wiki.osdev.org/UEFI)