pub fn print(message: &str) -> Result<i32, ()> {
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
