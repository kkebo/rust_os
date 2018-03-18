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

    println!("memory areas:");
    boot_info
        .memory_map_tag()
        .expect("Memory map tag required")
        .memory_areas()
        .for_each(|area| {
            println!(
                "    start: 0x{:x}, size: 0x{:x}",
                area.start_address(),
                area.size()
            )
        });

    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");
    println!("kernle sections:");
    elf_sections_tag.sections().for_each(|section| {
        println!(
            "    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.start_address(),
            section.size(),
            section.flags().bits()
        )
    });

    let kernel_start = elf_sections_tag
        .sections()
        .map(|s| s.start_address())
        .min()
        .unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .map(|s| s.end_address())
        .max()
        .unwrap();

    println!(
        "kernel_start: 0x{:x}, kernel_end: 0x{:x}",
        kernel_start, kernel_end
    );
    println!(
        "multiboot_start: 0x{:x}, multiboot_end: 0x{:x}",
        boot_info.start_address(),
        boot_info.end_address()
    );

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
