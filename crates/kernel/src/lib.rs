//! Minimal kernel entry point for Day 1 and syscall boundary for Day 2.

use abi::syscall::{errno, SysResult, Syscall};
use abi::HostAbi;
use userland::SyscallGate;

pub struct Kernel<'h, H: HostAbi> {
    host: &'h mut H,
    user_mem: Vec<u8>,
    last_exit_code: Option<i32>,
}

impl<'h, H: HostAbi> Kernel<'h, H> {
    pub fn new(host: &'h mut H) -> Self {
        Self {
            host,
            user_mem: Vec::new(),
            last_exit_code: None,
        }
    }

    pub fn boot(&mut self) {
        self.host.console_write("Kernel booting...\n");
        self.host
            .console_write("Welcome to rust-implement-linux-wasm\n");
        self.host.console_write("Type 'help' for commands.\n");
        self.host.console_write("> ");
    }

    pub fn handle_input(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() {
            self.host.console_write("> ");
            return;
        }

        if line == "help" {
            self.host
                .console_write("Commands: help, echo <text>, hello\n> ");
            return;
        }

        if let Some(rest) = line.strip_prefix("echo ") {
            self.host.console_write(rest);
            self.host.console_write("\n> ");
            return;
        }

        if line == "hello" {
            self.run_user_program(UserProgram::Hello);
            self.host.console_write("> ");
            return;
        }

        self.host.console_write("Unknown command. Type 'help'.\n> ");
    }

    pub fn handle_syscall(&mut self, num: usize, a0: usize, a1: usize, a2: usize) -> SysResult {
        match Syscall::from_u32(num as u32) {
            Some(Syscall::Write) => self.sys_write(a0, a1, a2),
            Some(Syscall::Exit) => self.sys_exit(a0),
            Some(Syscall::Yield) => errno::ENOSYS,
            None => errno::ENOSYS,
        }
    }

    fn sys_write(&mut self, fd: usize, buf_ptr: usize, len: usize) -> SysResult {
        if fd != 1 {
            return errno::EINVAL;
        }

        let text = match self.user_read(buf_ptr, len) {
            Some(bytes) => match std::str::from_utf8(bytes) {
                Ok(text) => text.to_owned(),
                Err(_) => return errno::EINVAL,
            },
            None => return errno::EINVAL,
        };

        self.host.console_write(&text);
        match isize::try_from(len) {
            Ok(count) => count,
            Err(_) => errno::EINVAL,
        }
    }

    fn sys_exit(&mut self, code: usize) -> SysResult {
        self.last_exit_code = Some(code as i32);
        0
    }

    fn user_read(&self, ptr: usize, len: usize) -> Option<&[u8]> {
        let end = ptr.checked_add(len)?;
        self.user_mem.get(ptr..end)
    }

    fn run_user_program(&mut self, program: UserProgram) {
        self.user_mem.clear();
        self.last_exit_code = None;
        let mut gate = KernelGate { kernel: self };
        match program {
            UserProgram::Hello => userland::hello(&mut gate),
        }
    }
}

enum UserProgram {
    Hello,
}

struct KernelGate<'k, 'h, H: HostAbi> {
    kernel: &'k mut Kernel<'h, H>,
}

impl<'k, 'h, H: HostAbi> SyscallGate for KernelGate<'k, 'h, H> {
    fn syscall(&mut self, num: usize, a0: usize, a1: usize, a2: usize) -> isize {
        self.kernel.handle_syscall(num, a0, a1, a2)
    }

    fn push_str(&mut self, s: &str) -> (usize, usize) {
        let ptr = self.kernel.user_mem.len();
        self.kernel.user_mem.extend_from_slice(s.as_bytes());
        (ptr, s.len())
    }
}

/// Kernel main entry point invoked by a host environment.
pub fn kmain<H: HostAbi>(host: &mut H) {
    let mut kernel = Kernel::new(host);
    kernel.boot();
}

/// Handle a single line of user input.
pub fn handle_input<H: HostAbi>(host: &mut H, line: &str) {
    let mut kernel = Kernel::new(host);
    kernel.handle_input(line);
}
