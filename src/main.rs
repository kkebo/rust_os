#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

use uefi::prelude::*;

#[entry]
fn efi_main(_image: uefi::Handle, mut st: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&mut st).expect_success("Failed to initialize utilities");

    st.stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");

    main();

    Status::SUCCESS
}

fn main() {
    log::info!("Hello, World{}", "!");

    loop {
        unsafe { asm!("hlt") }
    }
}
