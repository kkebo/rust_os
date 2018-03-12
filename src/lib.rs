#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main() {
    vga_buffer::clear_screen();
    println!("Hello, World{}", "!");
    println!("{}", {
        println!("inner");
        "outer"
    });

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
