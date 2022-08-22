use std::{fs};
use std::path::Path;
use bevy_inspector_egui::egui::TextBuffer;
use scan_dir::ScanDir;
use serde::{Deserialize, Serialize};
use crate::Registry;

#[derive(Eq, PartialEq, Debug, Clone, Default, Hash)]
pub struct Element {
    pub data: ElementData,
}

impl Element {
    pub const ELEMENT_PATH: &'static str = "sprites/elements/";

    pub const FROST_BOTTLE: Element = Element::new(ElementData::new("Frost Bottle", "frost_bottle", "Cold to the touch"));
    pub const YETI_WATER: Element = Element::new(ElementData::new("Yeti Water", "yeti_water", "A hydrating liquid with a strange stench"));
    pub const GLACIER_WATER: Element = Element::new(ElementData::new("Glacier Ice", "glacier_ice", "Your tongue is drawn to the frosty surface"));
    pub const LEGEND_DAIRY: Element = Element::new(ElementData::new("Legend Dairy", "legend_dairy", "Utterly Delicious. Legend speaks of the cow from which this heavenly cream comes from"));

    // note update the number when new elements are created
    pub const ELEMENTS: [Element; 4] = [
        Element::FROST_BOTTLE,
        Element::YETI_WATER,
        Element::GLACIER_WATER,
        Element::LEGEND_DAIRY,
    ];

    const fn new(element_data: ElementData) -> Self {
        Self {
            data: element_data,
        }
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Default, Hash)]
pub struct ElementData {
    pub name: &'static str,
    pub id: &'static str,
    pub desc: &'static str,
}

impl ElementData {
    pub const fn new(name: &'static str, id: &'static str, desc: &'static str) -> Self {
        Self {
            name,
            id,
            desc,
        }
    }

    pub fn path(&self) -> String {
        let path: String = format!("{}{}.png", Element::ELEMENT_PATH, self.id);
        path
    }
}

// util
fn create_id(name: &'static str) -> &'static str {
    unimplemented!()
}

mod tests {
    #[test]
    fn create_id_test() {}
}