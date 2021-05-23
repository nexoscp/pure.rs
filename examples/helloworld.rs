#![no_std]
#![no_main]

use pure::exit::{exit, ExitCode, OK};
use pure::print::print;

fn main() -> Result<(), ExitCode> {
    print("Hello World\n")?;
    Ok(())
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    match main() {
        Ok(_) => exit(OK),
        Err(c) => exit(c)
    }
}
