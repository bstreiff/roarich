#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub icon: String,

    // weapon type
    // item level
    //pub level_item,
    // rarity

    // phys damage (DamagePhys)
    pub damage_phys: u32,
    pub damage_mag: u32,
    pub delay_ms: u32,
    pub block_rate: u32,
    pub block: u32,
    pub defense_phys: u32,
    pub defense_mag: u32,
    // auto-attack time
    // delay

    // classes
    // min lvl requirement
    pub level_equip: u32,

    // stat bonuses
    pub materia_slot_count: u32,
    pub advanced_melds_permitted: bool,

    pub is_unique: bool,
    pub is_untradable: bool,
    pub can_be_hq: bool,
    pub dye_count: u32,
    pub is_crest_worthy: bool,
}
