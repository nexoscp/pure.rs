#![no_std]
#![no_main]
#![feature(asm)]
mod exit;
use core::panic::PanicInfo;
use crate::exit::{ExitCode, OK, exit};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    match main() {
        Ok(_) => exit(OK),
        Err(c) => exit(c)
    }
}

fn main() -> Result<(), ExitCode> {
    print("Hello World\n")?;
    Ok(())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(ExitCode::new(1))
}

fn print(message: &str) -> Result<i32, ()> {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1, // syscall number
            in("rdi") 1, // fd (stdout)
            in("rsi") message.as_ptr(),
            in("rdx") message.len(),
            out("rcx") _, // clobbered by syscalls
            out("r11") _, // clobbered by syscalls
            lateout("rax") ret,
        );
    }
    Ok(ret)
}

