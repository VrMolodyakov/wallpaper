use std::ffi::OsStr;
use std::{io, iter, env, process};
use std::os::windows::prelude::OsStrExt;
use winapi::ctypes::c_void;
use rand::distributions::{Alphanumeric, DistString};

use winapi::um::winuser::{SystemParametersInfoW, SPI_GETDESKWALLPAPER,SPIF_SENDCHANGE,SPIF_UPDATEINIFILE,SPI_SETDESKWALLPAPER};

const MAX_WINDOWS_PATH:usize = 260;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let image_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let image_path  = format!("{}{}{}{}", &config.file_path.to_string(),"/",image_name,".jpg");

    if let Err(err) = download_image(&image_path, &config.url){
        eprintln!("Couldn't download image: {err}");
        process::exit(1);
    }

    if let Err(err) = set_image(&image_path){
        eprintln!("Couldn't set new image: {err}");
        process::exit(1);
    }
    process::exit(0);
   

}



struct Config{
    file_path:String,
    url:String,
}

impl Config{
    fn build(mut args:impl Iterator<Item = String>) -> Result<Config, &'static str>{
        args.next();
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("the file path is not specified"),
        };
        let url = match args.next() {
            Some(arg) => arg,
            None => return Err("the image url is not specified"),
        };
        Ok(Config{file_path:path,url:url})
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

fn set_image(path:&str) ->Result<(),Box<dyn std::error::Error>>{
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
    println!("{}",file_name);
    let mut file = std::fs::File::create(file_name)?;
    reqwest::blocking::get(url)?.copy_to(&mut file)?;
    Ok(())

}