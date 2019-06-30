#![no_std]
#![no_main]

use uefi::prelude::*;

#[no_mangle]
pub extern "C" fn efi_main(_image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    // Initialize utilities (logging, memory allocation...)
    uefi_services::init(&st).expect_success("Failed to initialize utilities");

    st.stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");

    main();

    Status::SUCCESS
}

fn main() {
    log::info!("Hello, World{}", "!");

    loop {}
}
