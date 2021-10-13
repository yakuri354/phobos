// TODO Good abstractions
pub fn read_cr0() -> u64 {
    let mut res;
    unsafe { asm!("mov {}, cr0", out(reg) res) }
    return res;
}

pub fn read_cr3() -> u64 {
    let mut res;
    unsafe { asm!("mov {}, cr3", out(reg) res) }
    return res;
}

pub fn read_cr4() -> u64 {
    let mut res;
    unsafe { asm!("mov {}, cr4", out(reg) res) }
    return res;
}
