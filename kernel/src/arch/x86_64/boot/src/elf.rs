use goblin::elf::Elf;
use core::panic;
use x86_64::mm::paging::{PML4E, PDPT, PD, PT, PagingChainRef};

/// This function can only be called with Boot Services enabled
pub fn map_elf(raw: &[u8], pc: PagingChainRef) {
    let st = uefi_services::system_table();
    match Elf::parse(raw) {
        Ok(elf) => {
            if !elf.is_64 {
                panic!("Kernel is 32-bit")
            }
            for ph in elf.program_headers {
                if ph.p_type == goblin::elf::program_header::PT_LOAD {
                    if ph.p_filesz != 0x1000 {
                        panic!("Kernel program headers not properly aligned, found filesz {}", ph.p_filesz)
                    }

                }
            }
        }
        Err(e) => panic!("Kernel image is not a valid ELF executable file")
    }
}