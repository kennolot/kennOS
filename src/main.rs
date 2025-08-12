#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kennOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kennOS::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to kennOS!");

    // breakpoint exception
    kennOS::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

// panic handler test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kennOS::test_panic_handler(info)    
}