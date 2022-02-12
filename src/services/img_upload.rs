use crate::error::{AppError, Result};
use exif::{In, Reader, Tag};
use image::{guess_format, ImageFormat};
use std::fs::File;
use std::io::Cursor;
use uuid::Uuid;

pub fn img_upload(img_bytes: Vec<u8>, save_path:&'static str) -> Result<()> {
    let orientation = get_orientation(&img_bytes);
    println!("orientaiton {orientation}");
    let (format, ext) = get_format_and_ext(&img_bytes)?;
    let file_name = create_file_name(&ext);
    match image::load_from_memory_with_format(&img_bytes, format) {
        Ok(img) => {
            let new_img = img.thumbnail(300, 300).blur(2.0);
            let mut output = File::create(file_name).unwrap();
            new_img.write_to(&mut output, format).unwrap();
        }
        Err(_) => return Err(AppError::InvalidFileFormat),
    }
    Ok(())
}

pub fn img_rotate() {

}

pub fn get_orientation(img_bytes: &Vec<u8>) -> u32 {
    let mut buf = Cursor::new(img_bytes);
    let mut orientation = 1;
    if let Ok(exif) = Reader::new().read_from_container(&mut buf) {
        if let Some(o) = exif.get_field(Tag::Orientation, In::PRIMARY) {
            if let Some(v @ 1..=8) = o.value.get_uint(0) {
                orientation = v;
            }
        }
    }
    orientation
}

pub fn get_format_and_ext(img_bytes: &Vec<u8>) -> Result<(ImageFormat, &'static str)> {
    match guess_format(img_bytes) {
        Ok(ImageFormat::Png) => return Ok((ImageFormat::Png, "png")),
        Ok(ImageFormat::Jpeg) => return Ok((ImageFormat::Jpeg, "jpg")),
        Ok(ImageFormat::Gif) => return Ok((ImageFormat::Gif, "gif")),
        _ => return Err(AppError::InvalidFileFormat),
    };
}

pub fn create_file_name(ext:&'static str) -> String {
    format!("{}.{}", Uuid::new_v4().to_hyphenated(), ext) 
}

pub fn save_gcp() {

}