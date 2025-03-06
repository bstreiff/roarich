use egui::{
    load::{Bytes, BytesLoadResult, BytesLoader, BytesPoll, LoadError},
    mutex::Mutex,
    Vec2,
};
use image::{DynamicImage, ImageBuffer};
use image_dds::Surface;
use ironworks::{file::tex, Ironworks};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

fn read_dds(
    texture: tex::Texture,
    image_format: image_dds::ImageFormat,
) -> Result<DynamicImage, LoadError> {
    let surface = Surface {
        width: texture.width().into(),
        height: texture.height().into(),
        depth: texture.depth().into(),
        layers: match texture.kind() {
            tex::TextureKind::Cube => 6,
            tex::TextureKind::D2Array => texture.array_size().into(),
            _other => 1,
        },
        mipmaps: texture.mip_levels().into(),
        image_format,
        data: texture.data(),
    };

    if let Ok(decoded_surface) = surface.decode_rgba8() {
        if let Ok(image) = decoded_surface.to_image(0) {
            return Ok(image.into());
        }
    }

    Err(LoadError::Loading("dds read failure".into()))
}

fn read_bgra8(texture: tex::Texture) -> Result<DynamicImage, LoadError> {
    // copied out of boilmaster
    //
    // it sure would be nice if the image crate supported bgra

    let mut data = texture.data().clone();

    for i in 0..(data.len() / 4) {
        let x = data[i * 4];
        data[i * 4] = data[(i * 4) + 2];
        data[(i * 4) + 2] = x;
    }

    let buffer =
        ImageBuffer::from_raw(texture.width().into(), texture.height().into(), data).unwrap();

    Ok(DynamicImage::ImageRgba8(buffer))
}

#[derive(Default)]
pub struct AssetLoader {
    ironworks: Arc<Ironworks>,

    cache: Arc<Mutex<HashMap<String, BytesPoll>>>,
}

impl AssetLoader {
    pub const ID: &'static str = egui::generate_loader_id!(AssetLoader);

    pub fn new(ironworks: &Arc<Ironworks>) -> AssetLoader {
        Self {
            ironworks: ironworks.clone(),

            ..Default::default()
        }
    }
}

const PROTOCOL: &str = "asset://";

impl BytesLoader for AssetLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(&self, ctx: &egui::Context, uri: &str) -> BytesLoadResult {
        // we only know how to deal with assets
        let Some(path) = uri.strip_prefix(PROTOCOL) else {
            return Err(LoadError::NotSupported);
        };

        let mut cache = self.cache.lock();
        if let Some(cache_entry) = cache.get(uri).cloned() {
            return Ok(cache_entry.clone());
        } else {
            let Ok(texture) = self.ironworks.file::<tex::Texture>(path) else {
                return Err(LoadError::Loading("ironworks load error".into()));
            };

            if !matches!(texture.kind(), tex::TextureKind::D2) {
                return Err(LoadError::FormatNotSupported {
                    detected_format: Some(format!("texture kind: {:?}", texture.kind())),
                });
            }

            let texture_size = Vec2 {
                x: texture.width().into(),
                y: texture.height().into(),
            };

            let dynimage = match texture.format() {
                tex::Format::Bgra8Unorm => read_bgra8(texture)?,
                tex::Format::Bc1Unorm => read_dds(texture, image_dds::ImageFormat::BC1RgbaUnorm)?,
                tex::Format::Bc2Unorm => read_dds(texture, image_dds::ImageFormat::BC2RgbaUnorm)?,
                tex::Format::Bc3Unorm => read_dds(texture, image_dds::ImageFormat::BC3RgbaUnorm)?,
                tex::Format::Bc4Unorm => read_dds(texture, image_dds::ImageFormat::BC4RUnorm)?,
                tex::Format::Bc5Unorm => read_dds(texture, image_dds::ImageFormat::BC5RgUnorm)?,
                tex::Format::Bc6hFloat => read_dds(texture, image_dds::ImageFormat::BC6hRgbSfloat)?,
                tex::Format::Bc7Unorm => read_dds(texture, image_dds::ImageFormat::BC7RgbaUnorm)?,
                _ => {
                    return Err(LoadError::FormatNotSupported {
                        detected_format: Some(format!("texture format: {:?}", texture.format())),
                    });
                }
            };

            let mut bytes = Cursor::new(vec![]);
            if dynimage
                .write_to(&mut bytes, image::ImageFormat::WebP)
                .is_err()
            {
                return Err(LoadError::Loading(
                    "unable to convert texture to webp".into(),
                ));
            }
            let arc_bytes: Arc<[u8]> = bytes.clone().into_inner().into();

            let ready = BytesPoll::Ready {
                size: Some(texture_size),
                bytes: Bytes::Shared(arc_bytes.clone()),
                mime: Some("image/webp".to_string()),
            };

            cache.insert(uri.into(), ready.clone());
            ctx.request_repaint();

            Ok(ready)
        }
    }

    fn forget(&self, uri: &str) {
        let _ = self.cache.lock().remove(uri);
    }

    fn forget_all(&self) {
        self.cache.lock().clear();
    }

    fn byte_size(&self) -> usize {
        self.cache
            .lock()
            .values()
            .map(|entry| match entry {
                BytesPoll::Ready {
                    size: _,
                    bytes,
                    mime,
                } => bytes.len() + mime.as_ref().map_or(0, |m| m.len()),
                _ => 0,
            })
            .sum()
    }
}
