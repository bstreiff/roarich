#![warn(clippy::all, rust_2018_idioms)]

//use std::collections::HashMap;
use crate::data_provider::DataProvider;
use libxivdat::xiv_gearset::Gearset;
use serde_json::{Map, Value};

const XIVGEAR_ITEM_LABELS: [&str; 14] = [
    "Weapon",
    "OffHand",
    "Head",
    "Body",
    "Hand",
    "",
    "Legs",
    "Feet",
    "Ears",
    "Neck",
    "Wrist",
    "RingLeft",
    "RingRight",
    "SoulCrystal",
];

const CLASSJOB_NAMES: [&str; 43] = [
    "ADV", "GLA", "PGL", "MRD", "LNC", "ARC", "CNJ", "THM", "CRP", "BSM", "ARM", "GSM", "LTW",
    "WVR", "ALC", "CUL", "MIN", "BTN", "FSH", "PLD", "MNK", "WAR", "DRG", "BRD", "WHM", "BLM",
    "ACN", "SMN", "SCH", "ROG", "NIN", "MCH", "DRK", "AST", "SAM", "RDM", "BLU", "GNB", "DNC",
    "RPR", "SGE", "VPR", "PCT",
];

fn base_param_name(param: i32) -> &'static str {
    match param {
        6 => "piety",
        19 => "tenacity",
        22 => "dhit",
        27 => "crit",
        44 => "determination",
        45 => "skillspeed",
        46 => "spellspeed",
        _ => "",
    }
}

//
// {"name":"Machinist",
//  "sets": [ {"name":"Default Set",
//             "items":{"Weapon":{"id":42958,"materia":[{"id":41772},{"id":41772}]},
//                      "Head":{"id":44529,"materia":[{"id":41772},{"id":41771}]},
//                      "Body":{"id":44782,"materia":[{"id":41771},{"id":41771}]},
//                      "Hand":{"id":44783,"materia":[{"id":-1},{"id":-1}]},
//                      "Legs":{"id":42910,"materia":[{"id":-1},{"id":-1},{"id":-1},{"id":-1},{"id":-1}]}
//                      "Feet":{"id":42988,"materia":[{"id":41772},{"id":41773}]},
//                      "Ears":{"id":43083,"materia":[{"id":41772},{"id":41771}]},
//                      "Neck":{"id":44808,"materia":[{"id":41772},{"id":41772}]},
//                      "Wrist":{"id":43093,"materia":[{"id":-1},{"id":-1}]},
//                      "RingLeft":{"id":44818,"materia":[{"id":41771},{"id":41773}]},
//                      "RingRight":{"id":43098,"materia":[{"id":41771},{"id":41771}]}}}],
//  "level":100,
//  "job":"MCH"}
//

// xivgear doesn't understand base classes, only jobs.
//
// this information doesn't seem to be in the ClassJob table anywhere, but thankfully FFXIV will
// never get a job with a different base class ever again, so hardcoding this table is fine.
//
// (xivgear also doesn't seem to keep track of any items that are less than ilvl 290 (lowest lv70 gear),
// so exporting a gearset that isn't on a class with a job is probably of limited usefulness anyway...)
fn promote_to_job(class_job: u8) -> u8 {
    match class_job {
        // GLA -> PLD
        1 => 19,
        // PGL -> MNK
        2 => 20,
        // MRD -> WAR
        3 => 21,
        // LNC -> DRG
        4 => 22,
        // ARC -> BRD
        5 => 23,
        // CNJ -> WHM
        6 => 24,
        // THM -> BLM
        7 => 25,
        // ACN -> SMN
        26 => 27,
        // all others
        x => x,
    }
}

pub fn get_xivgear_json<T: DataProvider>(
    gearset: &Gearset,
    data_provider: &T,
) -> std::string::String {
    let mut items_map = Map::new();

    for (i, eq) in gearset.equipment.iter().enumerate() {
        // Skip belt slot and soul crystal
        if i == 5 || i == 13 {
            continue;
        }

        // xivgear doesn't seem to care if we put down an offhand item for a job that
        // doesn't use it

        let mut item_entry = Map::new();
        // xivgear assumes that all HQ-able gear is HQ.
        let item_id = if eq.item_id > 1000000 {
            eq.item_id - 1000000
        } else {
            eq.item_id
        };
        item_entry.insert("id".to_string(), Value::Number(item_id.into()));

        // We have to resolve materia (which is stored as class+grade) to the item id
        let mut materia_vec = Vec::with_capacity(5);
        let mut relic_stats = Map::new();

        for m in 0..eq.materia_types.len() {
            if eq.materia_types[m] != 0 {
                let materia_info = data_provider
                    .get_materia(eq.materia_types[m] as u32)
                    .unwrap();
                let materia_item_id = materia_info.item_id[eq.materia_grades[m] as usize];

                if materia_item_id != 0 {
                    // This is a normal materia
                    let mut materia_entry = Map::new();
                    materia_entry.insert("id".to_string(), Value::Number(materia_item_id.into()));
                    materia_vec.push(Value::Object(materia_entry));
                } else {
                    // This is not a normal materia, it's a stat bonus on a relic weapon. xivgear
                    // wants to have the stat bonuses, so look them up.
                    let stat_type = base_param_name(materia_info.base_param_id);
                    let stat_bonus = materia_info.base_param_value[eq.materia_grades[m] as usize];

                    relic_stats.insert(stat_type.to_string(), Value::Number(stat_bonus.into()));
                }
            }
        }
        item_entry.insert("materia".to_string(), Value::Array(materia_vec));
        if !relic_stats.is_empty() {
            item_entry.insert("relicStats".to_string(), Value::Object(relic_stats));
        }

        items_map.insert(
            XIVGEAR_ITEM_LABELS[i].to_string(),
            Value::Object(item_entry),
        );
    }

    let mut root_map = Map::new();
    root_map.insert("name".to_string(), Value::String(gearset.name.clone()));
    // TODO: Should we look at item equip levels to figure out if this is 70/80/90/100?
    // (xivgear doesn't support anything lower)
    root_map.insert("level".to_string(), Value::Number(100.into()));
    root_map.insert(
        "job".to_string(),
        Value::String(CLASSJOB_NAMES[promote_to_job(gearset.class_job) as usize].to_string()),
    );
    root_map.insert("items".to_string(), Value::Object(items_map));

    let root = Value::Object(root_map);

    root.to_string()
}
