use core::convert::From;

pub struct ExitCode(u8);
pub const OK:ExitCode = ExitCode(0);

impl ExitCode {
     pub fn new(value:u8) -> ExitCode { ExitCode(value) }
}

impl From<()> for ExitCode {
    fn from(_: ()) -> Self {
        ExitCode(0)
    }
}

// https://git.musl-libc.org/cgit/musl/tree/src/exit/_Exit.c
pub fn exit(code: ExitCode) -> ! {
    loop {
        unsafe {
            asm!(
                "syscall",
                in("rax") 60, // syscall number
                in("rdi") code.0 as i32,
                out("rcx") _, // clobbered by syscalls
                out("r11") _, // clobbered by syscalls
            );
        }
    }
}