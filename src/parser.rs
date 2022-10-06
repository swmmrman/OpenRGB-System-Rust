use openrgb::{
    data::Color,
    OpenRGB,
};

use serde_derive::Deserialize;
use home;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use std::error::Error;
use std::{fs,thread};
use std::time::Duration;
use tokio;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    sensors: HashMap<String, Sensor>,
    leds: HashMap<String, Led>
}
#[derive(Deserialize, Debug)]
struct Sensor {
    name: String,
    sensor_type: String,
    file: String,
    range: Vec<u32>,

}

#[derive(Deserialize, Debug)]
struct Led {
    name: String,
    id: usize,
    led_type: String,
    zones: Vec<usize>,
}

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
    let home = match home::home_dir() {
        Some(h) => h,
        None => PathBuf::from("/"),
    };
    change_sys_color(Color::new(255,255,255)).await?;
    let hf = format!("{}/.config/openrgb-monitor-rust/config.toml", home.display());

    let path = match std::fs::metadata(hf.to_string()).is_ok() {
        true => hf.to_string(),
        false => "config.toml".to_string(),
    };
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            println!("Error reading config: {}", e);
            exit(1);
        }
    };
    let config: Config = match toml::from_str::<Config>(&contents) {
        Ok(conf) => conf,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
    println!("Config = {:#?}", config);


    thread::sleep(Duration::from_secs(5));
    change_sys_color(Color::new(8,2,0)).await?;
    Ok(())
}