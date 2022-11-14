// Force cargo to link against crates that aren't (yet) referenced from Rust
// code (but are referenced from this crate's C code).
// https://github.com/rust-lang/cargo/issues/9391
extern crate logger;

/// Emulates an x86-64 processor's timestamp counter, as read by rdtsc and
/// rdtscp.
#[repr(C)]
#[allow(non_snake_case)]
pub struct Tsc {
    // TODO: rename and make non-pub when we drop C API
    pub cyclesPerSecond: u64,
}

impl Tsc {
    /// Returns the host system's native TSC rate, or None if it couldn't be found.
    ///
    /// WARNING: this is known to fail completely on some supported CPUs
    /// (particularly AMD), and can return the wrong value for others. i.e. this
    /// needs more work if we need to dependably get the host's TSC rate.
    /// e.g. see <https://github.com/shadow/shadow/issues/1519>.
    pub fn native_cycles_per_second() -> Option<u64> {
        todo!();
    }

    pub fn new(cycles_per_second: u64) -> Self {
        Self {
            cyclesPerSecond: cycles_per_second,
        }
    }

    fn set_rdtsc_cycles(&self, rax: &mut u64, rdx: &mut u64, nanos: u64) {
        // The multiply is guaranteed not to overflow since both operands are 64 bit.
        let cycles = u128::from(self.cyclesPerSecond) * u128::from(nanos) / 1_000_000_000;
        // *possible* that we'll wrap around here, but only after a very long
        // simulated time and/or a ridiculously fast clock. Wrapping is also
        // presumably what would happen on real hardware.
        let cycles = cycles as u64;
        *rdx = (cycles >> 32) & 0xff_ff_ff_ff;
        *rax = cycles & 0xff_ff_ff_ff;
    }

    const RDTSC: [u8; 2] = [0x0f, 0x31];
    const RDTSCP: [u8; 3] = [0x0f, 0x01, 0xf9];

    /// Updates registers to reflect the result of executing an rdtsc
    /// instruction at time `nanos`.
    pub fn emulate_rdtsc(&self, rax: &mut u64, rdx: &mut u64, rip: &mut u64, nanos: u64) {
        self.set_rdtsc_cycles(rax, rdx, nanos);
        *rip += Self::RDTSC.len() as u64;
    }

    /// Updates registers to reflect the result of executing an rdtscp
    /// instruction at time `nanos`.
    pub fn emulate_rdtscp(
        &self,
        rax: &mut u64,
        rdx: &mut u64,
        rcx: &mut u64,
        rip: &mut u64,
        nanos: u64,
    ) {
        self.set_rdtsc_cycles(rax, rdx, nanos);
        *rip += Self::RDTSCP.len() as u64;

        // rcx is set to IA32_TSC_AUX. According to the Intel developer manual
        // 17.17.2 "IA32_TSC_AUX Register and RDTSCP Support", "IA32_TSC_AUX
        // provides a 32-bit field that is initialized by privileged software with a
        // signature value (for example, a logical processor ID)." ... "User mode
        // software can use RDTSCP to detect if CPU migration has occurred between
        // successive reads of the TSC. It can also be used to adjust for per-CPU
        // differences in TSC values in a NUMA system."
        //
        // For now we just hard-code an arbitrary constant, which should be fine for
        // the stated purpose.
        // `hex(int(random.random()*2**32))`
        *rcx = 0x806eb479;
    }

    /// SAFETY: `ip` must be a dereferenceable pointer, pointing to the beginning
    /// of a valid x86_64 instruction, and `insn` must be a valid x86_64 instruction.
    unsafe fn ip_matches(ip: *const u8, insn: &[u8]) -> bool {
        // SAFETY:
        // * Caller has guaranteed that `ip` points to some valid instruction.
        // * Caller has guaranteed that `insn` is a valid instruction.
        // * No instruction can be a prefix of another, so `insn` can't be a prefix
        //   of some *other* instruction at `ip`.
        // * [`std::Iterator::all`] is short-circuiting.
        //
        // e.g. consider the case where `ip` points to a 1-byte `ret`
        // instruction, and the next byte of memory isn't accessible. That
        // single byte *cannot* match the first byte of `insn`, so we'll never
        // dereference `ip.offset(1)`, which would be unsound.
        insn.iter()
            .enumerate()
            .all(|(offset, byte)| unsafe { *ip.offset(offset as isize) == *byte })
    }

    /// Whether `ip` points to an rdtsc instruction.
    ///
    /// SAFETY: `ip` must be a dereferenceable pointer, pointing to the
    /// beginning of a valid x86_64 instruction.
    pub unsafe fn ip_is_rdtsc(ip: *const u8) -> bool {
        unsafe { Self::ip_matches(ip, &Self::RDTSC) }
    }

    // Whether `ip` points to an rdtscp instruction.
    ///
    /// SAFETY: `ip` must be a dereferenceable pointer, pointing to the
    /// beginning of a valid x86_64 instruction.
    pub unsafe fn ip_is_rdtscp(ip: *const u8) -> bool {
        unsafe { Self::ip_matches(ip, &Self::RDTSCP) }
    }
}

mod export {
    use super::*;

    /// Instantiate a TSC with the given clock rate.
    #[no_mangle]
    pub extern "C" fn Tsc_create(cycles_per_second: u64) -> Tsc {
        Tsc::new(cycles_per_second)
    }

    /// Updates `regs` to reflect the result of executing an rdtsc instruction at
    /// time `nanos`.
    #[no_mangle]
    pub extern "C" fn Tsc_emulateRdtsc(
        tsc: *const Tsc,
        rax: *mut u64,
        rdx: *mut u64,
        rip: *mut u64,
        nanos: u64,
    ) {
        let tsc = unsafe { tsc.as_ref().unwrap() };
        let rax = unsafe { rax.as_mut().unwrap() };
        let rdx = unsafe { rdx.as_mut().unwrap() };
        let rip = unsafe { rip.as_mut().unwrap() };
        tsc.emulate_rdtsc(rax, rdx, rip, nanos)
    }

    /// Updates `regs` to reflect the result of executing an rdtscp instruction at
    /// time `nanos`.
    #[no_mangle]
    pub extern "C" fn Tsc_emulateRdtscp(
        tsc: *const Tsc,
        rax: *mut u64,
        rdx: *mut u64,
        rcx: *mut u64,
        rip: *mut u64,
        nanos: u64,
    ) {
        let tsc = unsafe { tsc.as_ref().unwrap() };
        let rax = unsafe { rax.as_mut().unwrap() };
        let rdx = unsafe { rdx.as_mut().unwrap() };
        let rcx = unsafe { rcx.as_mut().unwrap() };
        let rip = unsafe { rip.as_mut().unwrap() };
        tsc.emulate_rdtscp(rax, rdx, rcx, rip, nanos)
    }

    /// Whether `buf` begins with an rdtsc instruction.
    #[no_mangle]
    pub extern "C" fn isRdtsc(ip: *const u8) -> bool {
        unsafe { Tsc::ip_is_rdtsc(ip) }
    }

    /// Whether `buf` begins with an rdtscp instruction.
    #[no_mangle]
    pub extern "C" fn isRdtscp(ip: *const u8) -> bool {
        unsafe { Tsc::ip_is_rdtscp(ip) }
    }
}
