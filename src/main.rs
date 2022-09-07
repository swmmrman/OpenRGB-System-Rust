use openrgb::{
    data::Color,
    OpenRGB,
};

use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OpenRGB::connect().await?;
    client.set_name("OpenRGB System Rust").await?;
    let _count = client.get_controller_count().await?;
    //println!("{}", count);
    
    let controller = client.get_controller(0).await?;
    let mut colors = controller.colors;
    for i in 0..117 {
        colors[i] = Color::new(0,0,0);
    }
    for i in 0..26 {
        colors[i] = Color::new(255,0,0);
    }
    for i in 26..36 {
        colors[i] = Color::new(0,0,255);
    }
    for i in 36..75 {
        colors[i] = Color::new(0,255,0);
    }
    for i in 75..79 {
        colors[i] = Color::new(255,200,0);
    }
    for i in 79..95 {
        colors[i] = Color::new(0,255,0);
    }
    for i in 95..112 {
        colors[i] = Color::new(0,255,255);
    }
    for i in 112..117 {
        colors[i] = Color::new(255,0,255);
    }
    let _led_count = colors.len();
    client.update_leds(0, colors).await?;
    //println!("{}", led_count);
    //println!("{:#?}", controller.leds);
    Ok(())
}
