//functionality to support an attribute that should fail

#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
// #![reexport_test_harness_main = "test_main"]

use blog_os::serial_print;
use core::panic::PanicInfo;
use blog_os::{QemuExitCode, exit_qemu, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //test_main();
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

//replaces test_runner
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
//note that when this test fails (it should!) compilation
//continues and QMEU is released


//test case to see if should_fails works 
// #[test_case]
// fn should_fail() {
//     serial_print!("should_panic::should_fail...\t");
//     assert_eq!(0, 1);
// }
