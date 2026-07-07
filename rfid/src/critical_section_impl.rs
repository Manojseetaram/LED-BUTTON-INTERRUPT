use critical_section::{Impl, RawRestoreState};

struct SingleCoreCriticalSection;
critical_section::set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let primask: u32;
        unsafe {
            core::arch::asm!("mrs {}, PRIMASK", out(reg) primask);
            core::arch::asm!("cpsid i");
        }
        (primask & 1) != 0
    }

    unsafe fn release(was_masked: RawRestoreState) {
        if was_masked {
            unsafe {
                core::arch::asm!("cpsie i");
            }
        }
    }
}
