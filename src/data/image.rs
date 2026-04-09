use std::{
    collections::HashMap,
    io::{BufWriter, Cursor},
    path::{Path, PathBuf},
    sync::LazyLock,
};

static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::CONFIG_PATH.join("images"));

#[derive(Debug)]
pub struct ImageSizes {
    pub original: Box<[u8]>,
    pub thumbnail: Box<[u8]>,
}

static RESIZED_IMAGES: LazyLock<HashMap<String, ImageSizes>> =
    LazyLock::new(|| create_resized_images().expect("Couldn't generate resized images"));

pub fn lazy_load() {
    _ = &*RESIZED_IMAGES;
}

pub fn get_image(name: &str, size_name: &str) -> Option<&'static [u8]> {
    let sizes = RESIZED_IMAGES.get(name)?;

    match size_name {
        "original" => Some(sizes.original.as_ref()),
        "thumbnail" => Some(sizes.thumbnail.as_ref()),
        _ => None,
    }
}

fn create_resized_images() -> anyhow::Result<HashMap<String, ImageSizes>> {
    let mut map = HashMap::new();

    for entry in std::fs::read_dir(&*PATH)? {
        let path = entry?.path();
        map.insert(
            path.file_stem().unwrap().to_string_lossy().into_owned(),
            create_resized_image(path.as_path())?,
        );
    }

    Ok(map)
}

fn create_resized_image(path: &Path) -> anyhow::Result<ImageSizes> {
    const THUMBNAIL_HEIGHT: u32 = 192;

    let original = image::open(path)?;
    let thumbnail = original.resize(
        ((original.width() as f32) / (original.height() as f32) * THUMBNAIL_HEIGHT as f32) as u32,
        THUMBNAIL_HEIGHT,
        image::imageops::FilterType::CatmullRom,
    );
    Ok(ImageSizes {
        original: image_to_png_buffer(original)?,
        thumbnail: image_to_png_buffer(thumbnail)?,
    })
}

fn image_to_png_buffer(img: image::DynamicImage) -> anyhow::Result<Box<[u8]>> {
    let mut buf = Vec::<u8>::new();
    img.write_to(
        BufWriter::new(Cursor::new(&mut buf)),
        image::ImageFormat::Png,
    )?;
    Ok(buf.into_boxed_slice())
}
