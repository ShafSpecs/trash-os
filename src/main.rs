#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(trash_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use trash_os::println;

// No mangle means the name of the function should 
// be left as is with no hashing (mangling) done to it
#[no_mangle]
pub extern "C" fn _start() -> ! { // actual entry point of the kernel (or any C program)
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    panic!("Some panic message");

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    trash_os::test_panic_handler(info)
}
