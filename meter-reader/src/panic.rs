use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("PANIC {}", info);
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
