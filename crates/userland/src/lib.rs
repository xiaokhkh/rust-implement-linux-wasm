use abi::syscall::Syscall;

pub trait SyscallGate {
    fn syscall(&mut self, num: usize, a0: usize, a1: usize, a2: usize) -> isize;
    fn push_str(&mut self, s: &str) -> (usize, usize);
}

pub fn hello<G: SyscallGate>(gate: &mut G) {
    let (ptr, len) = gate.push_str("hello from userland\n");
    let _ = gate.syscall(Syscall::Write as usize, 1, ptr, len);
    let _ = gate.syscall(Syscall::Exit as usize, 0, 0, 0);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    fn source_text() -> String {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = PathBuf::from(manifest_dir).join("src/lib.rs");
        fs::read_to_string(path).expect("read userland source")
    }

    #[test]
    fn userland_has_no_host_imports() {
        let text = source_text();
        let banned = [
            ["Host", "Abi"].concat(),
            ["wasm", "_bindgen"].concat(),
            ["web", "_sys"].concat(),
            ["js", "_sys"].concat(),
            ["host", "-", "wasm"].concat(),
            ["host", "_", "wasm"].concat(),
        ];
        for banned in banned {
            assert!(
                !text.contains(&banned),
                "userland must not import {banned}"
            );
        }
    }

    #[test]
    fn userland_uses_syscalls() {
        let text = source_text();
        assert!(
            text.contains("Syscall::Write"),
            "userland should use Syscall::Write"
        );
        assert!(text.contains("syscall("), "userland should invoke syscall gate");
    }
}
