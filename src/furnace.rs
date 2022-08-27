use bevy::prelude::Res;
use crate::element::Element;
use crate::registry::{FurnaceRecipeIden, Registry};

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct FurnaceRecipe {
    pub fuel : Element,
    pub object : Element,
    pub result : Element,
}

impl FurnaceRecipe {
    // format: fuel, object, result
    pub const GLACIER_ICE: FurnaceRecipe = FurnaceRecipe::new(Element::FROST_BOTTLE, Element::YETI_WATER, Element::GLACIER_ICE);

    //pub const TEST: FurnaceRecipe = FurnaceRecipe::new(Element::FIRE_PEPPER, Element::YETI_WATER, Element::LEGEND_DAIRY);

    pub const RECIPES: [FurnaceRecipe; 1] = [
        FurnaceRecipe::GLACIER_ICE,
        // FurnaceRecipe::TEST,
    ];

    pub const fn new(fuel: Element, object: Element, result: Element) -> Self {
        Self {
            fuel,
            object,
            result,
        }
    }

    pub fn id(&self) -> String {
        let id = format!("{}_{}", self.fuel.id, self.object.id);
        return id;
    }
}