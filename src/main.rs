use sysinfo::{System, SystemExt, CpuExt};
use openrgb::{
    data::Color,
    OpenRGB,
};
use std::{thread, time};
// use std::fs::File;
// use std::path::Path;
use std::error::Error;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio;
use std::collections::VecDeque;
use openrgb_system_rust;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let target_controller = 1;
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting handler");
    thread::sleep(time::Duration::from_secs(1));
    let client = OpenRGB::connect().await?;
    client.set_name("OpenRGB System Rust").await?;
    let keyboard = client.get_controller(target_controller).await?;
    let mut colors = keyboard.colors.to_vec();
    let cpu_file = openrgb_system_rust::get_cpu_file().unwrap();
    let keys = vec!(
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
        "Z", "X", "C", "V", "B", "N", "M", ",", ".", "/", 
        "Logo",
        "F1", "F4", "F5", "F2", "F3", "F6", "F7", "F8"
    );
    let indexs = openrgb_system_rust::get_key_indexs(keys, &keyboard.leds);
    let mut bg = Vec::new();
    for _ in  0..colors.len() {
        bg.push(Color::new(63,31,0));
    }
    colors = bg;
    client.update_leds(0, colors.to_vec()).await?;
    
    let mut sys = System::new_all();
    let mut cpu_vals: VecDeque<f32> = VecDeque::from([0.32; 10]);
    while running.load(Ordering::SeqCst) {
        print!("\r");
        //fans start at keys[21]
        for (i, fan) in openrgb_system_rust::get_fans().iter().enumerate() {
            let max_speeds = vec!(2250.0, 4800.0, 2000.0, 2250.0, 2250.0, 2200.0, 2200.0, 2200.0);
            let fan_led = indexs[i + 21];
            let fan_percent: f32 = fan / max_speeds[i];
            colors[fan_led] = openrgb_system_rust::get_color(fan_percent);
        }
        io::stdout().flush()?;
        sys.refresh_all();
        let mut i = 0;
        for core in sys.cpus() {
            colors[indexs[i]] = openrgb_system_rust::get_color(core.cpu_usage() / 100.0);
            i = i + 1; 
        }
        cpu_vals.pop_front();
        cpu_vals.push_back(((openrgb_system_rust::get_cpu_temp(&cpu_file) - 24.0)*1.4) / 100.0);
        let cpu_avg = cpu_vals.iter().sum::<f32>() / 10.0;
        colors[indexs[20]] = openrgb_system_rust::get_color(cpu_avg);
        io::stdout().flush().unwrap();
        client.update_leds(target_controller, colors.to_vec()).await?;
        thread::sleep(time::Duration::from_millis(100));
    }
    thread::sleep(time::Duration::from_millis(100));
    let exit_color = Color::new(63, 0, 0);
    for c in 1..client.get_controller_count().await? {
         let mut exit_colors = Vec::new();
         for _ in 0..client.get_controller(c).await?.colors.len() {
             exit_colors.push(exit_color);
         }
         client.update_leds(c, exit_colors).await?;
    }
    Ok(())
}
