//use sysinfo::{System, SystemExt};
use openrgb::{
    data::Color,
    OpenRGB,
};
use std::{thread, time, fs};
//use std::fs::File;
use std::error::Error;
use std::io;
use tokio;

fn get_cpu_file() -> Result<String, io::Error> {
    let mut cpufile = String::new();
    let zones = fs::read_dir("/sys/class/thermal/")?;
    for zone in zones {
        let type_path = format!("{}/type", &zone.as_ref().unwrap().path().display());
        let sensor_type = fs::read_to_string(type_path).expect("Error");
        if sensor_type == "x86_pkg_temp\n" {
            cpufile = format!("{}/temp", &zone.as_ref().unwrap().path().display());
        }
    }
    Ok(cpufile)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OpenRGB::connect().await?;
    client.set_name("OpenRGB System Rust").await?;
    let keyboard = client.get_controller(0).await?;
    //Lets store the current config to restore later.
    let orig_colors = keyboard.colors;
    let _orig_mode = keyboard.active_mode;

    println!("{}", get_cpu_file().unwrap());

    thread::sleep(time::Duration::from_secs(1));
    //client.update_mode(0,2);
    client.update_leds(0, orig_colors).await?;
    Ok(())
}
