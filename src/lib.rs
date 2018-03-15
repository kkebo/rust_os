#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(unique)]
#![no_std]

extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("Hello, World{}", "!");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    memory_map_tag.memory_areas().for_each(|area| {
        println!(
            "    start: 0x{:x}, size: 0x{:x}",
            area.start_address(), area.size()
        )
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
