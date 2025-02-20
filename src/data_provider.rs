use egui::ImageSource;
use std::error::Error;
use std::fmt;

use crate::model::Item;
use crate::model::Materia;

use image::error::ImageError;
use image_dds::error::{CreateImageError, SurfaceError};
use ironworks::Error as IWError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DataProviderError {
    DatabaseNotAvailable(&'static str),
    DatabaseError(&'static str),
    ObjectNotFound(&'static str),
    FieldTypeMismatch(&'static str),
    UnsupportedTextureType(&'static str),
    ImageDecodeError(&'static str),
    ImageEncodeError(&'static str),
    GenericImageError(&'static str),
}

impl fmt::Display for DataProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataProviderError::DatabaseNotAvailable(desc) => {
                write!(f, "Database not available: {}", desc)
            }
            DataProviderError::DatabaseError(desc) => write!(f, "Database error: {}", desc),
            DataProviderError::ObjectNotFound(desc) => write!(f, "Object not found: {}", desc),
            DataProviderError::FieldTypeMismatch(desc) => {
                write!(f, "Field type mismatch: {}", desc)
            }
            DataProviderError::UnsupportedTextureType(desc) => {
                write!(f, "Unsupported texture type: {}", desc)
            }
            DataProviderError::ImageDecodeError(desc) => {
                write!(f, "image decode error: {}", desc)
            }
            DataProviderError::ImageEncodeError(desc) => {
                write!(f, "image encode error: {}", desc)
            }
            DataProviderError::GenericImageError(desc) => {
                write!(f, "generic image error: {}", desc)
            }
        }
    }
}

impl std::error::Error for DataProviderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<IWError> for DataProviderError {
    fn from(iwerr: IWError) -> DataProviderError {
        match iwerr {
            ironworks::Error::NotFound(_) => DataProviderError::ObjectNotFound("object not found"),
            _ => DataProviderError::DatabaseError("unknown database error"),
        }
    }
}

impl From<SurfaceError> for DataProviderError {
    fn from(err: SurfaceError) -> DataProviderError {
        match err {
            SurfaceError::UnsupportedDdsFormat(_) => {
                DataProviderError::UnsupportedTextureType("unsupported DDS format")
            }
            _ => DataProviderError::ImageDecodeError("image decode error"),
        }
    }
}

impl From<CreateImageError> for DataProviderError {
    fn from(err: CreateImageError) -> DataProviderError {
        match err {
            _ => DataProviderError::ImageDecodeError("surface creation error"),
        }
    }
}

impl From<ImageError> for DataProviderError {
    fn from(err: ImageError) -> DataProviderError {
        match err {
            ImageError::Decoding(_) => DataProviderError::ImageDecodeError("image decode error"),
            ImageError::Encoding(_) => DataProviderError::ImageDecodeError("image encode error"),
            _ => DataProviderError::GenericImageError("unknown image failure"),
        }
    }
}

pub trait DataProvider {
    fn get_item(&self, item_id: u32) -> Result<Item, DataProviderError>;
    fn get_materia(&self, id: u32) -> Result<Materia, DataProviderError>;

    fn get_image(&self, path: &str) -> Result<ImageSource<'_>, DataProviderError>;
    fn get_ui_image_by_id(&self, id: u32) -> Result<ImageSource<'_>, DataProviderError>;
}
