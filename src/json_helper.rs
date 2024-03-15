use eframe::egui::{self};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use uuid::Uuid;
use std::collections::HashMap;

use crate::map_element;
use map_element::*;

#[derive(Serialize, Deserialize)]
struct LocationJson {
    loc_name: String,
    kind: String,
    x: Number,
    y: Number
}

#[derive(Serialize, Deserialize)]
struct LocationsJson {
    light_world: Vec<LocationJson>,
    dark_world: Vec<LocationJson>
}

pub fn load (json_data: &str) -> HashMap<Uuid, map_element::MapElement> {
    let mut items: HashMap<Uuid, MapElement> = HashMap::new();

    let locations: LocationsJson = serde_json::from_str(json_data).expect("Expected JSON");
    for loc in locations.light_world {
        let pos = egui::Pos2::new(loc.x.as_f64().unwrap() as f32, loc.y.as_f64().unwrap() as f32);

        let item = MapElement {
            loc_name: loc.loc_name,
            pos: pos,
            kind: str_to_kind(&loc.kind),
            id: Uuid::new_v4(),
            checked: false
        };
        items.insert(item.id, item);
    }

    for loc in locations.dark_world {
        let pos = egui::Pos2::new(loc.x.as_f64().unwrap() as f32 + 750.0, loc.y.as_f64().unwrap() as f32);

        let item = map_element::MapElement {
            loc_name: loc.loc_name,
            pos: pos,
            kind: str_to_kind(&loc.kind),
            id: Uuid::new_v4(),
            checked: false
        };
        items.insert(item.id, item);
    }

    return items;
}