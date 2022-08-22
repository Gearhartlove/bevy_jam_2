use bevy::prelude::Res;
use scan_dir::ScanDir;
use serde::{Serialize, Deserialize};
use crate::element::Element;
use crate::registry::{FurnaceRecipeIden, Registry};

pub struct FurnaceRecipe {
    pub data: FurnaceData,
}

impl FurnaceRecipe {
    pub const GLACIER_ICE: FurnaceRecipe = FurnaceRecipe::new(FurnaceData::new(Element::FROST_BOTTLE, Element::YETI_WATER, Element::GLACIER_WATER));

    pub const RECIPES: [FurnaceRecipe; 1] = [
        FurnaceRecipe::GLACIER_ICE,
    ];

    const fn new(data: FurnaceData) -> Self {
        Self { data }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct FurnaceData {
    pub fuel : Element,
    pub object : Element,
    pub result : Element,
}

pub fn get_result(fuel : Element, object : Element, registry : &Res<Registry>) -> Option<Element> {
    let iden = FurnaceRecipeIden::new(fuel, object);
    if let Some(fr)  = registry.furnace_recipe_registry.get(&iden) {
        Some(fr.result.clone())
    } else {
        None
    }
}

impl FurnaceData {
    pub const fn new(fuel: Element, object: Element, result: Element) -> Self {
        Self {
            fuel,
            object,
            result,
        }
    }

    pub fn id(&self) -> String {
        let id = format!("{}_{}", self.fuel.data.id, self.object.data.id);
        return id;
    }
}