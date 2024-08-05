mod placeholder;

use rascii_art::{charsets, render_image_to, RenderOptions};
use rspotify::model::FullTrack;
use std::{collections::HashMap, error::Error};
use yansi::Paint;

use crate::ui::utils::{center_string, screen_width};

fn image(url: &String, cache: &mut HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    // Check if the image is already cached
    if let Some(image) = cache.get(url) {
        return Ok(image.clone());
    }

    // Download image file
    let mut img_buffer: Vec<u8> = Vec::new();
    ureq::get(url)
        .call()?
        .into_reader()
        .read_to_end(&mut img_buffer)?;

    // Interpret file as an image
    let img = image::load_from_memory(&img_buffer)?;

    // Convert image to text
    let mut buffer = String::new();
    render_image_to(
        &img,
        &mut buffer,
        &RenderOptions::new()
            .width(screen_width().try_into().unwrap())
            .colored(true)
            .charset(charsets::BLOCK),
    )?;

    // Update cache
    cache.insert(url.clone(), buffer.clone());

    Ok(buffer)
}

fn artists(track: &FullTrack) -> String {
    track
        .artists
        .iter()
        .map(|artist| artist.name.clone())
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn summary(track: &FullTrack) -> String {
    format!("{} - {}", track.name, artists(track))
}

pub fn display(track: &FullTrack, cache: &mut HashMap<String, String>) -> String {
    let image =
        image(&track.album.images[0].url, cache).unwrap_or(String::from(if screen_width() >= 48 {
            placeholder::IMAGE_48
        } else {
            placeholder::IMAGE_32
        }));
    let name = format!("{}", center_string(&track.name).bold());
    let artists = format!("{}", center_string(&artists(track)).italic());

    format!("{}\n{}\n{}", image, name, artists)
}
