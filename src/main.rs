#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ebi::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ebi::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ebi::memory;
    use x86_64::{
        structures::paging::{MapperAllSizes, Page},
        VirtAddr,
    };

    println!("Hello World{}", "!");
    ebi::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // trigger a stack overflow
    // stack_overflow();

    // let ptr = 0x204832 as *mut u32;
    // unsafe { let _x = *ptr; };
    // println!("read worked");
    // unsafe { *ptr = 42; };
    // println!("write worked");

    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // let mut frame_allocator = memory::EmptyFrameAllocator;
    // let mut frame_allocator = unsafe {
    //     memory::BootInfoFrameAllocator::init(&boot_info.memory_map);
    // };

    // map an unused page
    // let page = Page::containing_address(VirtAddr::new(0));
    // let page: Page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) }

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
