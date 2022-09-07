use std::ffi::OsStr;
use std::{io, iter};
use std::os::windows::prelude::OsStrExt;
use winapi::ctypes::c_void;

use winapi::um::winuser::{SystemParametersInfoW, SPI_GETDESKWALLPAPER,SPIF_SENDCHANGE,SPIF_UPDATEINIFILE,SPI_SETDESKWALLPAPER};

const MAX_WINDOWS_PATH:usize = 260;

fn main() {
    let url = "https://cdna.artstation.com/p/assets/images/images/053/497/996/large/muhammet-feyyaz-plaguemarine.jpg?1662370257";
    let file_name = "download.jpg";
    if let Err(err) = download_image(file_name,url){
        println!("{:?}",err)
    }
}

struct Config{
    file_path:String,
    url:String,
}

impl Config{
    fn build(mut args:impl Iterator<Item = String>) -> Result<Config,&'static str>{
        args.next();
        Ok(Config{file_path:String::from(""),url:String::from("")})
    }
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

fn set_path(path:&str) ->Result<(),Box<dyn std::error::Error>>{
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

fn download_image(file_name:&str,url:&str) -> Result<(),Box<dyn std::error::Error>>{
    let mut file = std::fs::File::create(file_name)?;
    reqwest::blocking::get(url)?.copy_to(&mut file)?;
    Ok(())

}