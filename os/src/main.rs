#![no_std]
#![no_main]
/// to make a rust binary that runs on baremetal
/// we cannot use the standard lib which uses
/// references to existing operating systems
use core::panic::PanicInfo;
// this function is called on panic!
// it returns the `never` type as it
// should never return anything
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// maind does not make sense without the 
/// underlying runtime that calls it, so we overwrite
/// the os entrypoint with a custom _start fn.
///
/// the no_mangle bit ensures that the rust compiler
/// outputs a function with the name _start and not 
/// some cryptic _ZN3blog_os4_start7hb173fedf945531caE 
/// symbol to give every function a unique name.
///
/// Will need to provide the explicite fn name to the linker 
/// later. The "C" part tells the compiler that it should use 
/// the C calling convention for this function instead of the 
/// unspecified Rust calling convention. _start is the default
/// entrypoint for most systems.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
