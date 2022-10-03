use openrgb::{
    data::Color,
    OpenRGB,
};

use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OpenRGB::connect().await?;
    let lights = client.get_controller(1).await?;
    let mut colors = Vec::new();
    for _ in 0..lights.colors.len() {
        colors.push(Color::new(255,255,255));
    }
    client.update_leds(1, colors).await?;
    Ok(())
}