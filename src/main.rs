#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });
    for i in 0..100 {
        println!("{}", 2 * rec_fib(i));
    }
    loop {}
}

fn rec_fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        rec_fib(n - 1) + rec_fib(n - 2)
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
