#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(veer::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use veer::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("||   / /  //___) ) //___) ) //  ) )");
    println!("||  / /  //       //       //");
    println!("|| / /  //       //       //");
    println!("||/ /  ((____   ((____   //");

    veer::init();

    #[cfg(test)]
    test_main();

    veer::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    veer::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    veer::test_panic_handler(info)
}

// example test case
#[test_case]
fn example_test() {
    assert_eq!(1, 1);
}