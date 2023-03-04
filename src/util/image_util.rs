use anyhow::Result;
use image::io::Reader;
use std::any::{type_name, Any};
use std::fs;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ImageInfo {
    height: u32,
    width: u32,
    size: u64,
    r#type: String,
}

pub fn get_image_info(img_path: &str) -> Result<ImageInfo> {
    let reader = Reader::open(img_path)?;
    let reader = reader.with_guessed_format()?;
    let mut r#type = String::new();
    if let Some(t) = reader.format() {
        if let Some(tt) = t.extensions_str().first() {
            r#type = String::from(*tt);
        }
    }
    let image = reader.decode()?;
    let size = fs::metadata(img_path)?;
    Ok(ImageInfo {
        height: image.height(),
        width: image.width(),
        size: size.len(),
        r#type,
    })
}

#[cfg(test)]
mod test {
    use crate::util::image_util::get_image_info;
    use std::time;

    #[test]
    fn test_image_info() {
        let now = time::Instant::now();

        let info = get_image_info("/Users/simman/Pictures/cat_avatar_01.gif");
        println!("{:?}", time::Instant::now() - now);
        // assert_eq!(info)
        println!("{:?}", info);
    }
}
