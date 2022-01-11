use crate::{data::late_init::LateInit, sync::irq_lock::IRQLocked};
use boot_lib::PHYS_MAP_OFFSET;
use core::slice::from_raw_parts;
use lazy_static::lazy_static;
use core::cmp::min;
use log::{error, info};
use x86_64::{
    instructions::tables::{lidt, sgdt, sidt},
    structures::{
        gdt::GlobalDescriptorTable,
        idt::{
            HandlerFunc, InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode,
            PageFaultHandlerFunc,
        },
    },
};
use x86_64::instructions::tables::lgdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut prev_idt = sidt();
        prev_idt.base += PHYS_MAP_OFFSET;
        let mut idt = unsafe { prev_idt.base.as_ptr::<InterruptDescriptorTable>().read() };

        idt.page_fault.set_handler_fn(page_fault);
        idt.double_fault.set_handler_fn(double_fault);
        idt.breakpoint.set_handler_fn(breakpoint);

        idt
    };
}

pub fn init() {
    let mut gdt = sgdt();
    gdt.base += PHYS_MAP_OFFSET;
    unsafe { lgdt(&gdt) }
    IDT.load();
}

extern "x86-interrupt" fn page_fault(frame: InterruptStackFrame, code: PageFaultErrorCode) {
    error!("Page fault occured");
    error!("{:#?}", frame);
    error!("Code: {:?}", code);
    panic!("Page Fault!")
}

extern "x86-interrupt" fn double_fault(frame: InterruptStackFrame, _code: u64) -> ! {
    error!("Double fault occured");
    error!("{:#?}", frame);
    panic!("Double Fault!")
}

extern "x86-interrupt" fn breakpoint(frame: InterruptStackFrame) {
    info!("Waiting for debugger");
    loop {}
}
