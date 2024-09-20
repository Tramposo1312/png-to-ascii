extern crate image;

use image::{GenericImageView, ImageBuffer, Rgba, Pixel};
use std::env;
use std::fs::File;
use std::io::Write;

const TARGET_WIDTH: u32 = 150;
const TARGET_HEIGHT: u32 = 120;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_image> <output.txt>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let img = image::open(input_path)?;
    
    // Resize the image
    let resized = img.resize_exact(TARGET_WIDTH, TARGET_HEIGHT, image::imageops::FilterType::Lanczos3);

    let ascii_chars = vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    let mut ascii_art = String::new();

    for y in 0..TARGET_HEIGHT {
        for x in 0..TARGET_WIDTH {
            let pixel = resized.get_pixel(x, y);
            let intensity = pixel.to_luma().0[0] as f32 / 255.0;
            let ascii_index = (intensity * (ascii_chars.len() - 1) as f32) as usize;
            ascii_art.push(ascii_chars[ascii_index]);
        }
        ascii_art.push('\n');
    }

    let mut output_file = File::create(output_path)?;
    output_file.write_all(ascii_art.as_bytes())?;

    println!("ASCII art has been saved to {}", output_path);

    Ok(())
}