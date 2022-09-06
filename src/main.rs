use std::ffi::OsStr;
use std::{io, iter};
use std::os::windows::prelude::OsStrExt;
use std::ptr;
use winapi::ctypes::c_void;

use winapi::um::winuser::{SystemParametersInfoW, SPI_GETDESKWALLPAPER,SPIF_SENDCHANGE,SPIF_UPDATEINIFILE,SPI_SETDESKWALLPAPER};

const MAX_WINDOWS_PATH:usize = 260;

fn main() {
    let wallpaper_path = set_by_path("C:\\Users\\Loken\\Desktop\\материалы\\wallp\\test.jpg").unwrap();
}

fn get_current() ->Result<String,Box<dyn std::error::Error>>{
    unsafe{
        let buffer :[u16;MAX_WINDOWS_PATH] = [0;MAX_WINDOWS_PATH];
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

fn set_by_path(path:&str) ->Result<(),Box<dyn std::error::Error>>{
    unsafe{
        let path = OsStr::new(path).encode_wide().chain(iter::once(0)).collect::<Vec<u16>>();
        let successful = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 1;
        if successful {
            Ok(())
        } else {
            Err(io::Error::last_os_error().into())
        }
    }
    
}