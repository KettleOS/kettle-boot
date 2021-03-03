#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use kettle_boot_api::vga::*;
use uefi::prelude::*;

#[entry]
fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
	Status::SUCCESS
}
