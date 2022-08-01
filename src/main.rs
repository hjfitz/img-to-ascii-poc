use std::env;
use std::fs;
use std::fs::File;

use ansi_term::Color::RGB;
use ansi_term::Style;
use image::imageops::FilterType;
use image::GenericImageView;
use ansi_to_tui::IntoText;


const BASE_URL: &str = "https://i.scdn.co/image/ab67616d00001e029b59ddd08f920ae0b0bdaf53";

#[tokio::main]
async fn main() {
    let response = reqwest::get(BASE_URL).await.unwrap();

    let raw_img_bytes = response.bytes().await.unwrap();
    let raw_img = image::load_from_memory(&raw_img_bytes);

    if raw_img.is_err() {
        panic!("{}", raw_img.as_ref().err().unwrap());
    }

    let img = raw_img.unwrap();

    let pallete: [char; 7] = [' ', '.', '/', '*', '#', '$', '@'];
    let mut y = 0;

    let mut art = vec![];
    let small_img = img.resize_exact(100, 50, FilterType::Nearest);
    for p in small_img.pixels() {
        if y != p.1 {
            art.push(Style::new().paint("\n").to_string());
            y = p.1;
        }

        let r = p.2 .0[0] as f32;
        let g = p.2 .0[1] as f32;
        let b = p.2 .0[2] as f32;
        let k = r * 0.3 + g * 0.59 + b * 0.11;
        let character = ((k / 255.0) * (pallete.len() - 1) as f32).round() as usize;

        let custom_char = pallete[character];

        let coloured_char = RGB(r as u8, g as u8, b as u8).paint(custom_char.to_string()).to_string();

        art.push(coloured_char);
    }

    println!("{}", art.join(""));
}
