#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ebi::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ebi::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ebi::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
