#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(try_trait)]
#![no_std]
#![no_main]

#[macro_use]
mod print;
mod uefi;

use core::panic::PanicInfo;

pub(crate) static mut UEFI_SYSTEM_TABLE: Option<&'static uefi::SystemTable> = None;

#[no_mangle]
pub extern "win64" fn efi_main(
    _handle: uefi::Handle,
    system_table: &'static uefi::SystemTable,
) -> uefi::Status {
    unsafe { UEFI_SYSTEM_TABLE = Some(&system_table) };

    println!("UEFI header: {:#?}", system_table.get_header());

    main();

    uefi::Status::Success
}

fn main() {
    println!("Hello, World{}", "!");

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("panic occurred: {:?}", s);
    } else {
        println!("panic occurred");
    }

    loop {}
}
