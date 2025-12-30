#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate bin_proto;

#[no_mangle]
fn main() {}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "alloc")]
struct Allocator;

#[cfg(feature = "alloc")]
unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {
        unimplemented!()
    }
}

#[cfg(feature = "alloc")]
#[global_allocator]
static GLOBAL: Allocator = Allocator;
