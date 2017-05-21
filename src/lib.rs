#![no_std]
#![feature(lang_items)]


#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {*(0xb8000 as *mut _) = 0x0A52;}
    unsafe {*(0xb8002 as *mut _) = 0x0A55;}
    unsafe {*(0xb8004 as *mut _) = 0x0A53;}
    unsafe {*(0xb8006 as *mut _) = 0x0A54;}

    loop {}
}


#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}


#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {loop{}}
