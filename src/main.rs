use openrgb::{
    data::Color,
    OpenRGB,
};
use std::{thread, time};

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
    let mut new_colors = vec![];
    for _ in 0..117 {
        new_colors.push(Color::new(255,0,0));
    }
    client.update_leds(0, new_colors.to_vec()).await?;
    thread::sleep(time::Duration::from_secs(1));
    //client.update_mode(0,2);
    client.update_leds(0, orig_colors).await?;
    Ok(())
}
