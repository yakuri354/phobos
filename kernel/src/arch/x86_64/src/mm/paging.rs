extern crate alloc;

use modular_bitfield::prelude::*;
use crate::PhysicalAddress;
use core::convert::TryFrom;
use alloc::boxed::Box;

pub trait PageTable {
    fn init_into(into: Box<[u64; 512]>) -> Box<Self>;
    /// The caller must ensure validity of data in the array
    unsafe fn from_raw_unchecked(data: Box<[u64; 512]>) -> Box<Self>;
}

#[repr(transparent)]
pub struct PML4E([u64; 512]);

impl PageTable for PML4E {
    fn init_into(into: Box<[u64; 512]>) -> Box<Self> {
        let uninit = unsafe { Self::from_raw_unchecked(into) };
        for i in uninit.0.iter() {
            let temp = RawPML4E::new();
            temp.set_writable(true);
            temp.set_non_non_exec(true);
            *i = temp;
        }
        uninit
    }
    unsafe fn from_raw_unchecked(data: Box<[u64; 512]>) -> Box<Self> {
        Box::from_raw(Box::into_raw(data) as *mut Self)
    }
}

#[repr(transparent)]
pub struct PDPT([u64; 512]);

impl PageTable for PDPT {
    fn init_into(into: Box<[u64; 512]>) -> Box<Self> {
        let uninit = unsafe { Self::from_raw_unchecked(into) };
        for i in uninit.0.iter() {
            *i = RawPDPTE::new().into()
        }
        uninit
    }
    unsafe fn from_raw_unchecked(data: Box<[u64; 512]>) -> Box<Self> {
        Box::from_raw(Box::into_raw(data) as *mut Self)
    }
}

#[repr(transparent)]
pub struct PD([u64; 512]);

impl PageTable for PD {
    fn init_into(into: Box<[u64; 512]>) -> Box<Self> {
        let uninit = unsafe { Self::from_raw_unchecked(into) };
        for i in uninit.0.iter() {
            *i = RawPDE::new().into()
        }
        uninit
    }
    unsafe fn from_raw_unchecked(data: Box<[u64; 512]>) -> Box<Self> {
        Box::from_raw(Box::into_raw(data) as *mut Self)
    }
}

#[repr(transparent)]
pub struct PT([u64; 512]);

impl PageTable for PT {
    fn init_into(into: Box<[u64; 512]>) -> Box<Self> {
        let uninit = unsafe { Self::from_raw_unchecked(into) };
        for i in uninit.0.iter() {
            *i = RawPTE::new().into()
        }
        uninit
    }
    unsafe fn from_raw_unchecked(data: Box<[u64; 512]>) -> Box<Self> {
        Box::from_raw(Box::into_raw(data) as *mut Self)
    }
}

pub struct PagingChainRef(Box<PML4E>, Box<PDPT>, Box<PD>, Box<PT>);

impl PagingChainRef {
    fn from_slice(raw: Box<[u8; 0x4000]>) -> PagingChainRef {
        for i in raw.array_chunks() {

        }
    }
}

enum AccessMode {
    R,
    RW,
    RWX,
    RX,
}

enum PagePermissions {
    Superuser,
    User,
}

enum PageTableEntry {
    Page(PageEntry),
    NextPageTable(NextPageTableEntry),
}

struct NextPageTableEntry {
    present: bool,
    access: AccessMode,
    permissions: PagePermissions,
    write_trough: bool,
    cache_disabled: bool,
    accessed: bool,
    next_table_address: PhysicalAddress,
}

enum PageEntrySize {
    GByte,
    TwoMByte,
    FourKByte,
}

struct PageProtectionKey(u8);

struct PageEntry {
    size: PageEntrySize,
    access: AccessMode,
    permissions: PagePermissions,
    write_trough: bool,
    cache_disabled: bool,
    accessed: bool,
    dirty: bool,
    pat: bool,
    global: bool,
    address: PhysicalAddress,
    key: PageProtectionKey,
}

#[bitfield]
#[repr(u64)]
pub(crate) struct RawPML4E {
    present: bool,
    writable: bool,
    usermode: bool,
    write_trough: bool,
    cache_disabled: bool,
    accessed: bool,
    #[skip]
    _ign: B1,
    #[skip]
    _res: B1,
    // Always zero
    #[skip]
    _ign2: B4,
    pdpt_pa: B40,
    #[skip]
    _ign3: B11,
    non_exec: bool,
}

#[bitfield]
#[repr(u64)]
struct RawPDPTE {
    present: bool,
    writable: bool,
    usermode: bool,
    write_trough: bool,
    cache_disabled: bool,
    accessed: bool,
    #[skip]
    _ign: B1,
    huge_page: bool,
    #[skip]
    _ign2: B4,
    page_dir_pa: B40,
    #[skip]
    _ign3: B11,
    non_exec: bool,
}

#[bitfield]
#[repr(u64)]
struct RawPDPTEBigPage {
    present: bool,
    writable: bool,
    usermode: bool,
    write_trough: bool,
    cache_disabled: bool,
    accessed: bool,
    dirty: bool,
    #[skip]
    _huge_page: bool,
    // Always true
    global: bool,
    #[skip]
    _ign1: B3,
    pat_mem_type: bool,
    #[skip]
    _res1: B17,
    page_pa: B21,
    #[skip]
    _res2: B1,
    #[skip]
    _ign2: B7,
    protect_key: B4,
    non_exec: bool,
}

#[bitfield]
#[repr(u64)]
struct RawPDE {
    present: bool,
    writable: bool,
    usermode: bool,
    write_trough: bool,
    cache_disabled: bool,
    accessed: bool,
    #[skip]
    _ign: B1,
    huge_page: bool,
    #[skip]
    _ign2: B4,
    page_dir_pa: B40,
    #[skip]
    _ign3: B11,
    non_exec: bool,
}

#[bitfield]
#[repr(u64)]
struct RawPDEBigPage {
    present: bool,
    writable: bool,
    usermode: bool,
    write_trough: bool,
    cache_disabled: bool,
    accessed: bool,
    dirty: bool,
    #[skip]
    _huge_page: bool,
    // Always true
    global: bool,
    #[skip]
    _ign1: B3,
    pat_mem_type: bool,
    #[skip]
    _res1: u8,
    page_pa: B30,
    #[skip]
    _res2: B1,
    #[skip]
    _ign2: B7,
    protect_key: B4,
    non_exec: bool,
}

#[bitfield]
#[repr(u64)]
struct RawPTE {
    present: bool,
    writable: bool,
    usermode: bool,
    write_trough: bool,
    cache_disabled: bool,
    accessed: bool,
    dirty: bool,
    pat_mem_type: bool,
    global: bool,
    #[skip]
    _ign1: B3,
    page_pa: B40,
    #[skip]
    _ign2: B7,
    protect_key: B4,
    non_exec: bool,
}
