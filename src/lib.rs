#![no_std]
#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]

extern crate rlibc;
extern crate spin;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {*(0xb8000 as *mut _) = 0x0A52;}
    unsafe {*(0xb8002 as *mut _) = 0x0A55;}
    unsafe {*(0xb8004 as *mut _) = 0x0A53;}
    unsafe {*(0xb8006 as *mut _) = 0x0A54;}

    vga_buffer::WRITER.lock().put_byte(b'7');

    loop {}
}


#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}


#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {loop{}}
