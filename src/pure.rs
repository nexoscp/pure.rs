use core::panic::PanicInfo;

use crate::exit::{exit, ExitCode};

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    exit(ExitCode::new(1))
}
