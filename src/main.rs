#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(try_trait)]
#![feature(unique)]
#![no_std]
#![no_main]

extern crate compiler_builtins;
extern crate fixedvec;

#[macro_use]
mod print;
mod uefi;

pub(crate) static mut UEFI_SYSTEM_TABLE: Option<&'static uefi::SystemTable> = None;

#[no_mangle]
pub extern "win64" fn uefi_main(
    handle: uefi::Handle,
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

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}
