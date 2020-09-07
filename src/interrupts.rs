use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}
pub fn init_idt() {
    IDT.load()
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame)
}

#[test_case]
fn test_breakpoint_exception() {
    // By checking that the execution continues afterwards, 
    // we verify that our breakpoint handler is working correctly.
    x86_64::instructions::interrupts::int3();
}