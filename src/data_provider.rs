use std::error::Error;
use std::fmt;

use crate::model::Item;
use crate::model::Materia;

use ironworks::Error as IWError;

#[derive(Debug)]
pub enum DataProviderError {
    DatabaseNotAvailable(&'static str),
    DatabaseError(&'static str),
    ObjectNotFound(&'static str),
    FieldTypeMismatch(&'static str),
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

pub trait DataProvider {
    fn get_item(&self, item_id: u32) -> Result<Item, DataProviderError>;
    fn get_materia(&self, id: u32) -> Result<Materia, DataProviderError>;
}
