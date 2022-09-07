use sysinfo::{System, SystemExt, CpuExt};
use openrgb::{
    data::Color,
    OpenRGB,
};
use std::{thread, time, fs};
use std::fs::File;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OpenRGB::connect().await?;
    client.set_name("OpenRGB System Rust").await?;
    let keyboard = client.get_controller(0).await?;
    //Lets store the current config to restore later.
    let orig_colors = keyboard.colors;
    let _orig_mode = keyboard.active_mode;


    thread::sleep(time::Duration::from_secs(1));
    //client.update_mode(0,2);
    client.update_leds(0, orig_colors).await?;
    Ok(())
}
