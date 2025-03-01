#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

mod vga_buffer;
mod keyboard;
mod memory;
mod allocator;
mod llm;
mod chat;

use crate::chat::ChatInterface;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialize the OS components
    println!("Initializing Bare Metal LLM OS...");
    
    // Initialize memory
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    // Initialize heap memory
    memory::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");
    
    // Initialize keyboard
    keyboard::init();
    
    println!("Memory and keyboard initialized!");
    println!("Starting LLM Chat Interface...");
    
    // Start the chat interface
    let mut chat = ChatInterface::new();
    chat.start_chat();
    
    // If the chat interface exits, we'll show the history and halt
    chat.show_history();
    
    println!("LLM OS session ended. Halting system.");
    hlt_loop();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
