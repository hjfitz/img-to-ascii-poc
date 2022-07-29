use std::env;
use std::fs;
use std::fs::File;
use std::io::{copy, Cursor};

use ansi_term::ANSIStrings;
use ansi_term::Color::RGB;
use ansi_term::Style;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImageView;
use ansi_to_tui::IntoText;


const BASE_URL: &str = "https://i.scdn.co/image/ab67616d00001e029b59ddd08f920ae0b0bdaf53";

#[tokio::main]
async fn main() {
    let object_path = "foo/test.png";

    let response = reqwest::get(BASE_URL).await.unwrap();

    let content_type = response
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let extension = content_type.replace("image/", ".");

    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    let object_prefix = &object_path[..object_path.rfind('/').unwrap()];
    let object_name = format!("tmp{}", extension);
    let output_dir = format!(
        "{}/{}",
        env::current_dir().unwrap().to_str().unwrap().to_string(),
        object_prefix
    );
    fs::create_dir_all(output_dir.clone()).unwrap();

    let output_fname = format!("{}/{}", output_dir, object_name);

    let mut dest = File::create(output_fname.clone()).unwrap();

    let mut content = Cursor::new(response.bytes().await.unwrap());
    copy(&mut content, &mut dest).unwrap();

    // read the file and print it

    let raw_img = image::open(output_fname);

    if raw_img.is_err() {
        panic!("{}", raw_img.as_ref().err().unwrap());
    }

    let img = raw_img.unwrap();
    let resolution = 5;

    let pallete: [char; 7] = [' ', '.', '/', '*', '#', '$', '@'];
    let mut y = 0;
    // let mut art = String::new();
    /*let small_img = img.resize(
        img.width() / resolution,
        img.height() / resolution,
        FilterType::Nearest,
    );
    */

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
        //luminosidade
        let k = r * 0.3 + g * 0.59 + b * 0.11;
        let character = ((k / 255.0) * (pallete.len() - 1) as f32).round() as usize;

        let custom_char = pallete[character];

        let coloured_char = RGB(r as u8, g as u8, b as u8).paint(custom_char.to_string()).to_string();

        art.push(coloured_char);
    }

    art.join("").as_bytes().to_vec().into_text();

    //let painted = ANSIStrings(&art);

    println!("{}", art.join(""));
}
