#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    //loop {}
    let ret = main();
    exit(ret)
}

fn main() -> u8 {
    print("Hello World\n");
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(1)
}

fn print(message: &str) -> i32 {
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
    ret
}

// https://git.musl-libc.org/cgit/musl/tree/src/exit/_Exit.c
fn exit(code: u8) -> ! {
    loop {
        unsafe {
            asm!(
            "syscall",
            in("rax") 60, // syscall number
            in("rdi") code as i32,
            out("rcx") _, // clobbered by syscalls
            out("r11") _, // clobbered by syscalls
            );
        }
    }
}