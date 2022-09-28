//use sysinfo::{System, SystemExt};
use openrgb::{
    data::Color,
    data::LED,
    OpenRGB,
};
use std::{thread, time, fs};
//use std::fs::File;
use std::error::Error;
use std::io::{self, Write};
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

fn get_cpu_temp(path: &str) -> f32 {
    let temp: String = fs::read_to_string(path)
        .expect(&format!("File not found {}", path));
    let temp = temp.trim().parse::<f32>().unwrap();
    temp / 1000.0
}

fn get_color(val: f32) -> Color {
    let val = val * 2.0;
    println!("{}", val);
    let r = std::cmp::max(std::cmp::min((val * 255.0) as u8, 255), 0);
    let g = std::cmp::max(std::cmp::min((510.0 - val * 255.0) as u8, 255), 0);
    let b = 0;
    Color::new(r,g,b)
}

fn get_key_indexs(keys: Vec<char>, leds: &Vec<LED>) -> Vec<usize> {
    let mut indexs = Vec::new();
    let mut led_names = Vec::new();
    for led in leds {
        let led_name = led.name.to_string();
        if led_name == "Logo" {
            led_names.push(led_name);
        }
        else {
            led_names.push(led_name[5..].to_string());
        }
    }
    for key in keys {
        let index = led_names.iter().position(|x| x == &key.to_string()).unwrap();
        indexs.push(index);
    }
    indexs
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let running = true;
    let client = OpenRGB::connect().await?;
    client.set_name("OpenRGB System Rust").await?;
    let keyboard = client.get_controller(0).await?;
    //Lets store the current config to restore later.
    let orig_colors = keyboard.colors.to_vec();
    let mut colors = keyboard.colors.to_vec();
    let _orig_mode = keyboard.active_mode;
    let cpu_file = get_cpu_file().unwrap();
    let keys = vec!(
        'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';',
        'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/'
    );
    let indexs = get_key_indexs(keys, &keyboard.leds);
    println!("{:?}", indexs);
    let mut bg = Vec::new();
    for _ in  0..colors.len() {
        bg.push(Color::new(128,64,0));
    }
    colors = bg;
    for index in indexs {
        colors[index] = get_color(0.10);
    }
    client.update_leds(0, colors).await?;
    while running {
        print!("\rCPU Temp: {}", get_cpu_temp(&cpu_file));
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(250));
    }
    thread::sleep(time::Duration::from_secs(1));
    //client.update_mode(0,2);
    client.update_leds(0, orig_colors).await?;
    Ok(())
}
