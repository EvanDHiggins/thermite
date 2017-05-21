#![no_std]
#![feature(lang_items)]


#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {*(0xb8000 as *mut _) = 0x0146;}
    unsafe {*(0xb8001 as *mut _) = 0x0146;}
    unsafe {*(0xb8002 as *mut _) = 0x0146;}
    unsafe {*(0xb8003 as *mut _) = 0x0146;}
}


#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}


#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {loop{}}
