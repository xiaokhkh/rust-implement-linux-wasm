//! Syscall ABI shared between the kernel and userland.

/// Syscall numbers.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Syscall {
    Write = 1,
    Exit = 2,
    Yield = 3,
}

impl Syscall {
    pub fn from_u32(num: u32) -> Option<Self> {
        match num {
            1 => Some(Syscall::Write),
            2 => Some(Syscall::Exit),
            3 => Some(Syscall::Yield),
            _ => None,
        }
    }
}

pub mod errno {
    pub const EINVAL: isize = -22;
    pub const ENOSYS: isize = -38;
}

pub type SysResult = isize;
