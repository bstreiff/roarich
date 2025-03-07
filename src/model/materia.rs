#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Materia {
    pub id: u32,

    pub item_id: Vec<u32>,
    pub base_param_id: i32,
    pub base_param_value: Vec<i16>,
}
