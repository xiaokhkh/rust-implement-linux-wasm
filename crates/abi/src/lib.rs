//! Host ABI shared between the kernel and its hosts.
//!
//! Keeping the ABI here lets the kernel stay platform-agnostic while hosts
//! implement the minimal set of services the kernel needs.

/// Minimal surface area the kernel expects from its host environment.
pub trait HostAbi {
    /// Write a string to the host console/log output.
    fn console_write(&mut self, s: &str);
}

/// Signature for the kernel entry point expected by hosts.
pub type KMainFn<H> = fn(&mut H)
where
    H: HostAbi;
