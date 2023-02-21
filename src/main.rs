#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer; 
mod serial; 

//use blog_os::println;
use core::panic::PanicInfo;

//a trait that all the future testables will share-in this case printing will be 'inherited'


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", " testing is not working");

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

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

/*
#[test_case]
fn trivial_assertion() {
    //serial_print!("trivial assertion... ");  remove prints b/c the trait print was implemented!
    assert_eq!(1, 1);
    //serial_println!("[ok]");
}*/

/*
#[test_case]
fn trivial_assertion_2() {
    //serial_print!("trivial assertion 2... ");
    assert_eq!(1000, 1000);
    //serial_println!("[ok]");
}*/

