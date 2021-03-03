#![no_std]
#![feature(const_fn)]

use kettle_boot_api::vga::*;

#[no_mangle]
pub extern "C" fn efi_main() -> ! {
	// test
	write_char(b'a', VgaColor::White);
	loop {}
}
