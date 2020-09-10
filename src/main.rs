#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ebi::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ebi::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    ebi::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // trigger a stack overflow
    // stack_overflow();

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ebi::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ebi::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ebi::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
