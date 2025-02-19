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
    pub damage_phys: u16,
    pub damage_mag: u16,
    pub delay_ms: u16,
    pub block_rate: u16,
    pub block: u16,
    pub defense_phys: u16,
    pub defense_mag: u16,
    pub cast_time: u8,
    pub cooldown: u16,
    // auto-attack time
    // delay

    // classes
    // min lvl requirement
    pub level_equip: u8,

    // stat bonuses
    pub materia_slot_count: u8,
    pub advanced_melds_permitted: bool,

    pub is_unique: bool,
    pub is_untradable: bool,
    pub can_be_hq: bool,
    pub dye_count: u8,
    pub is_crest_worthy: bool,
}
