#![no_std]
#![no_main]
#![feature(abi_efiapi)]

extern crate alloc;

mod arch;
mod view;

use core::panic::PanicInfo;
use uefi::prelude::entry;

#[entry]
fn efi_main(
    image: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>,
) -> uefi::Status {
    unsafe {
        uefi::alloc::init(system_table.boot_services());
    }
    system_table.stdout().clear().unwrap();
    uefi_services::init(&mut system_table).unwrap();
    view::State::new(image, system_table).run();
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
