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
    Ok(())
}
