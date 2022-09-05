use std::io;
use std::ptr;
use winapi::ctypes::c_void;

use winapi::um::winuser::{SystemParametersInfoW, SPI_GETDESKWALLPAPER};

fn main() {
    let wallpaper_path = get_current().unwrap();
    println!("{}",wallpaper_path);
    print!("end...")
}

fn get_current() ->Result<String,Box<dyn std::error::Error>>{
    unsafe{
        let buffer :[u16;260] = [0;260];
        let successful = SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            buffer.len() as u32,
            buffer.as_ptr() as *mut c_void,
            0,
        ) == 1;
        if successful {
            let path = String::from_utf16(&buffer)?
                // removes trailing zeroes from buffer
                .trim_end_matches('\x00')
                .into();
            Ok(path)
        } else {
            Err(io::Error::last_os_error().into())
        }

    }
}
