use openrgb::{
    data::Color,
    OpenRGB,
};

use std::process::exit;
use std::error::Error;
use std::{fs,thread};
use std::time::Duration;
use tokio;

async fn change_sys_color(color: Color) -> Result<(), Box<dyn Error>> {
    let client = OpenRGB::connect().await?;
    let lights = client.get_controller(1).await?;
    let mut colors = Vec::new();
    for _ in 0..lights.colors.len() {
        colors.push(color);
    }
    client.update_leds(1, colors).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    change_sys_color(Color::new(255,255,255)).await?;
    let path = match std::fs::metadata("~/.config/openrgb-monintor-rust/config.toml") {
        Ok(_) => "~/.config/openrgb-monintor-rust/config.toml",
        Err(_) => "config.toml",
    };
    
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            println!("Error reading config: {}", e);
            exit(1);
        }
    };
    println!("Config = {:#?}", contents);


    thread::sleep(Duration::from_secs(5));
    change_sys_color(Color::new(8,2,0)).await?;
    Ok(())
}