#![no_std]
#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]

extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;


#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    println!("multiboot_information_address: 0x{:x}", multiboot_information_address);

    let boot_info = unsafe{multiboot2::load(multiboot_information_address)};

    loop {}
}


#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}


#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(
    fmt: core::fmt::Arguments,
    file: &'static str,
    line: u32
    ) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("     {}", fmt);
    loop {}
}
