use sysinfo::{System, SystemExt, CpuExt};
use openrgb::{
    data::Color,
    data::LED,
    OpenRGB,
};
use std::{thread, time, fs};
// use std::fs::File;
// use std::path::Path;
use std::error::Error;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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
fn get_fans() -> Vec<u64> {
    let mut fans: Vec<u64>= Vec::new();
    let sensor_packs = fs::read_dir("/sys/class/hwmon/").unwrap();
    for sensor in sensor_packs {
        let sensor_name_file = format!("{}/name", &sensor.as_ref().unwrap().path().display()); 
        let sensor_name = fs::read_to_string(sensor_name_file).expect("File not found");
        println!("{}", sensor_name);
        if sensor_name == "nct6687\n" {
            for i in 1..=8 {
                let fan_path = format!("{}/fan{}_input", &sensor.as_ref().unwrap().path().display(), i);
                let fanspeed = fs::read_to_string(fan_path).unwrap();
                fans.push(fanspeed.trim().parse::<u64>().unwrap());
            }
            break;
        }
    }
    fans
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting handler");
    thread::sleep(time::Duration::from_secs(1));
    let client = OpenRGB::connect().await?;
    client.set_name("OpenRGB System Rust").await?;
    let keyboard = client.get_controller(0).await?;
    let chroma = client.get_controller(1).await?;
    let mut syscolors = chroma.colors.to_vec();
    for i in 0..syscolors.len() {
        syscolors[i] = Color::new(0,0,0);
    }
    client.update_leds(1, syscolors.to_vec()).await?;
    let mut colors = keyboard.colors.to_vec();
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
    while running.load(Ordering::SeqCst) {
        for fan in get_fans() {
            print!("Speed: {}", fan)
        }
        println!("");
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
    thread::sleep(time::Duration::from_millis(100));
    let exit_color = Color::new(63, 0, 0);
    for c in 0..client.get_controller_count().await? {
        let mut exit_colors = Vec::new();
        for _ in 0..client.get_controller(c).await?.colors.len() {
            exit_colors.push(exit_color);
        }
        client.update_leds(c, exit_colors).await?;
    }
    Ok(())
}
