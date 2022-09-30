use sysinfo::{System, SystemExt, CpuExt};
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
    let r = std::cmp::max(std::cmp::min((val * 255.0) as u8, 255), 0);
    let g = std::cmp::max(std::cmp::min((510.0 - val * 255.0) as u8, 255), 0);
    let b = 0;
    Color::new(r,g,b)
}

fn get_key_indexs(keys: Vec<&str>, leds: &Vec<LED>) -> Vec<usize> {
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
    thread::sleep(time::Duration::from_secs(5));
    let running = true;
    let client = OpenRGB::connect().await?;
    client.set_name("OpenRGB System Rust").await?;
    let keyboard = client.get_controller(0).await?;
    //Lets store the current config to restore later.
    let orig_colors = keyboard.colors.to_vec();
    let mut colors = keyboard.colors.to_vec();
    //let logo = colors[get_key_indexs(vec!("Logo"), &keyboard.leds)[0]];
    let _orig_mode = keyboard.active_mode;
    let cpu_file = get_cpu_file().unwrap();
    let keys = vec!(
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
        "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", "Logo"
    );
    let indexs = get_key_indexs(keys, &keyboard.leds);
    let mut bg = Vec::new();
    for _ in  0..colors.len() {
        bg.push(Color::new(128,64,0));
    }
    colors = bg;
    client.update_leds(0, colors.to_vec()).await?;
    
    let mut sys = System::new_all();
    while running {
        sys.refresh_all();
        let mut i = 0;
        for core in sys.cpus() {
            colors[indexs[i]] = get_color(core.cpu_usage() / 100.0);
            i = i + 1; 
        }
        colors[indexs[20]] = get_color(((get_cpu_temp(&cpu_file) - 30.0)*1.4) / 100.0);
        io::stdout().flush().unwrap();
        client.update_leds(0, colors.to_vec()).await?;
        thread::sleep(time::Duration::from_millis(100));
    }
    thread::sleep(time::Duration::from_millis(16));
    //client.update_mode(0,2);
    client.update_leds(0, orig_colors).await?;
    Ok(())
}
