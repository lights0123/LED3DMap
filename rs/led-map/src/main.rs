use std::fs::read_dir;

use image::io::Reader as ImageReader;

use led_map::{compute_light_pos, read_base_frame};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = ImageReader::open("frames/base.png")?.decode()?;
    let base = read_base_frame(&base);
    let mut frames: Vec<_> = read_dir("frames")?
        .filter_map(|file| file.ok())
        .map(|file| file.path())
        .filter(|file| file.file_name().map_or(true, |name| name != "base.png"))
        .collect();
    frames.sort_unstable();
    frames.into_iter().try_for_each(|file| {
        let lit = ImageReader::open(&file)?.decode()?;
        match compute_light_pos(&base, &lit) {
            Some((x, y, max_difference)) => {
                println!("{}, {}, {}", x, y, max_difference);
            }
            None => println!(),
        }
        Ok(())
    })
}
