#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use svm_kernel::{exit_qemu, init, println, QemuExitCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    svm_kernel::hlt_loop();
}

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();

    svm_kernel::hlt_loop();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("===== should_panic test =====");
    for test in tests {
        test();
        println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn should_fail() {
    assert_eq!(0, 1);
}
