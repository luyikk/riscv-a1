#![feature(panic_info_message)]
#![no_std]
#![no_main]

mod panic_def;
mod sbi;
mod console;


use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    warn!("hello,world!");
    info!("Hello, world!");
    debug!("Hello, world!");
    trace!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
   extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}