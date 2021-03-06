#![no_std]
#![no_main]
#![feature(custom_test_frameworks)] // https://github.com/rust-lang/rfcs/blob/master/text/2318-custom-test-frameworks.md
#![test_runner(svm_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]
#![feature(test)]
/*
 * Followed the tutorial here: https://os.phil-opp.com
 * TODO: Replace builtin memcpy, memset with optimized one
 */

/* TODO:
 * Write bootloader myself to be able to enable
 * mmx,sse & float features!
 * Should also solve the lto linktime warning
 */

/*
 * This kernel has been tested on an AMD x64 processor
 * family: 0x17h, model: 0x18h
 */

use svm_kernel::mylog::LOGGER;

use bootloader::bootinfo;
use bootloader::entry_point;

extern crate alloc;
/*
 * KERNEL MAIN
 * The macro entry_point creates the nomangle _start func for us and checks that
 * the given function has the correct signature
 */
//TODO: Reset rsp to start of stack
entry_point!(kernel_main);
fn kernel_main(_boot_info: &'static bootinfo::BootInfo) -> ! {

    // Init & set logger level
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let rsp: u64;
    unsafe {
        asm!("mov {}, rsp", out(reg) rsp);
    };
    log::info!("Reached kernel! rsp: {:x}", rsp);

    // Initialize routine for kernel
    svm_kernel::init(_boot_info);

    // // This func gets generated by cargo test
    // #[cfg(test)]
    // test_main();

    // Busy loop don't crash
    // log::info!("Quitting kernel...");
    // svm_kernel::exit_qemu(svm_kernel::QemuExitCode::Success);
    log::info!("Kernel going to loop now xoxo");
    svm_kernel::hlt_loop();
}

/*
 * KERNEL PANIC HANDLER
 * Not used in cargo test
 */
//TODO: Implement a bare metal debugger
// https://lib.rs/crates/gdbstub
// https://sourceware.org/gdb/onlinedocs/gdb/Remote-Protocol.html
// TODO: Make panic handler print stuff without a global lock
// If an error occurs while reading memory inside the print lock
// a deadlock occurs
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    svm_kernel::println!("{}", info);
    svm_kernel::hlt_loop();
}
