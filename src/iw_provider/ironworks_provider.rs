use crate::data_provider::{DataProvider, DataProviderError};
use crate::directories;
use crate::iw_provider::asset_loader::AssetLoader;
use crate::model::{Item, Materia};

use egui::ImageSource;
use ironworks::{
    excel::{Excel, Field, Language},
    sqpack::{Install, SqPack},
    Ironworks,
};
use std::sync::Arc;

#[derive(Default)]
pub struct IronworksProvider {
    ironworks: Arc<Ironworks>,
}

impl IronworksProvider {
    pub fn new() -> Self {
        let install = Install::at(
            directories::find_install()
                .expect("FFXIV install not found")
                .as_path(),
        );

        let ironworks = Arc::new(Ironworks::new().with_resource(SqPack::new(install)));

        Self {
            ironworks: ironworks,

            ..Default::default()
        }
    }

    pub fn install_bytes_loader(&self, ctx: &egui::Context) {
        if !ctx.is_loader_installed(AssetLoader::ID) {
            ctx.add_bytes_loader(std::sync::Arc::new(AssetLoader::new(
                &self.ironworks.clone(),
            )));
        }
    }
}

fn field_to_string(field: Field) -> Result<String, DataProviderError> {
    match field {
        Field::String(x) => Ok(x.to_string()),
        Field::Bool(x) => Ok(format!("{}", x)),
        Field::I8(x) => Ok(format!("{}", x)),
        Field::I16(x) => Ok(format!("{}", x)),
        Field::I32(x) => Ok(format!("{}", x)),
        Field::I64(x) => Ok(format!("{}", x)),
        Field::U8(x) => Ok(format!("{}", x)),
        Field::U16(x) => Ok(format!("{}", x)),
        Field::U32(x) => Ok(format!("{}", x)),
        Field::U64(x) => Ok(format!("{}", x)),
        Field::F32(x) => Ok(format!("{}", x)),
    }
}

fn field_to_u16(field: Field) -> Result<u16, DataProviderError> {
    match field {
        Field::String(_) => Err(DataProviderError::FieldTypeMismatch(
            "conversion of string to u16",
        )),
        Field::Bool(x) => Ok(x as u16),
        Field::I8(x) => Ok(x as u16),
        Field::I16(x) => Ok(x as u16),
        Field::I32(x) => Ok(x as u16),
        Field::I64(x) => Ok(x as u16),
        Field::U8(x) => Ok(x as u16),
        Field::U16(x) => Ok(x as u16),
        Field::U32(x) => Ok(x as u16),
        Field::U64(x) => Ok(x as u16),
        Field::F32(x) => Ok(x as u16),
    }
}

fn field_to_u32(field: Field) -> Result<u32, DataProviderError> {
    match field {
        Field::String(_) => Err(DataProviderError::FieldTypeMismatch(
            "conversion of string to u32",
        )),
        Field::Bool(x) => Ok(x as u32),
        Field::I8(x) => Ok(x as u32),
        Field::I16(x) => Ok(x as u32),
        Field::I32(x) => Ok(x as u32),
        Field::I64(x) => Ok(x as u32),
        Field::U8(x) => Ok(x as u32),
        Field::U16(x) => Ok(x as u32),
        Field::U32(x) => Ok(x as u32),
        Field::U64(x) => Ok(x as u32),
        Field::F32(x) => Ok(x as u32),
    }
}

fn field_to_i16(field: Field) -> Result<i16, DataProviderError> {
    match field {
        Field::String(_) => Err(DataProviderError::FieldTypeMismatch(
            "conversion of string to i16",
        )),
        Field::Bool(x) => Ok(x as i16),
        Field::I8(x) => Ok(x as i16),
        Field::I16(x) => Ok(x as i16),
        Field::I32(x) => Ok(x as i16),
        Field::I64(x) => Ok(x as i16),
        Field::U8(x) => Ok(x as i16),
        Field::U16(x) => Ok(x as i16),
        Field::U32(x) => Ok(x as i16),
        Field::U64(x) => Ok(x as i16),
        Field::F32(x) => Ok(x as i16),
    }
}

fn field_to_i32(field: Field) -> Result<i32, DataProviderError> {
    match field {
        Field::String(_) => Err(DataProviderError::FieldTypeMismatch(
            "conversion of string to i32",
        )),
        Field::Bool(x) => Ok(x as i32),
        Field::I8(x) => Ok(x as i32),
        Field::I16(x) => Ok(x as i32),
        Field::I32(x) => Ok(x as i32),
        Field::I64(x) => Ok(x as i32),
        Field::U8(x) => Ok(x as i32),
        Field::U16(x) => Ok(x as i32),
        Field::U32(x) => Ok(x as i32),
        Field::U64(x) => Ok(x as i32),
        Field::F32(x) => Ok(x as i32),
    }
}

// ui/icon/051000/051474_hr1.tex
fn ui_icon_path(id: u32) -> String {
    format!("ui/icon/{:0>6}/{:0>6}_hr1.tex", id - (id % 1000), id)
}

impl DataProvider for IronworksProvider {
    fn get_item(&self, item_id: u32) -> Result<Item, DataProviderError> {
        // HQ items are represented as ids above 1000000
        if item_id >= 1000000 {
            let nq_item = self.get_item(item_id - 1000000)?;

            return Ok(Item {
                id: item_id,
                name: format!("{} (HQ)", nq_item.name),

                ..nq_item
            });
        }

        let excel =
            Excel::new(Arc::clone(&self.ironworks)).with_default_language(Language::English);
        let items = excel.sheet("Item")?;
        let row = items.row(item_id)?;

        Ok(Item {
            id: item_id,
            name: field_to_string(row.field(9)?)?,
            icon: ui_icon_path(field_to_u32(row.field(10)?)?),

            ..Default::default()
        })
    }

    fn get_materia(&self, id: u32) -> Result<Materia, DataProviderError> {
        let excel =
            Excel::new(Arc::clone(&self.ironworks)).with_default_language(Language::English);
        let items = excel.sheet("Materia")?;
        let row = items.row(id)?;

        let mut item_id_vec = Vec::with_capacity(16);
        let mut base_param_value_vec = Vec::with_capacity(16);
        for i in 0..16 {
            item_id_vec.push(field_to_u32(row.field(i)?)?);
            base_param_value_vec.push(field_to_i16(row.field(i + 17)?)?);
        }

        Ok(Materia {
            id: id,
            item_id: item_id_vec,
            base_param_id: field_to_i32(row.field(16)?)?,
            base_param_value: base_param_value_vec,
        })
    }

    fn get_image(&self, path: &str) -> Result<ImageSource<'_>, DataProviderError> {
        // TODO: it would be nice if ironworks had a method to check for file existance before we return a uri
        Ok(ImageSource::Uri(format!("asset://{}", path).into()))
    }

    fn get_ui_image_by_id(&self, id: u32) -> Result<ImageSource<'_>, DataProviderError> {
        self.get_image(&ui_icon_path(id))
    }
}
