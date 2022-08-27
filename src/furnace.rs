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
    pub const GLACIER_ICE: FurnaceRecipe = FurnaceRecipe::new(Element::FROZEN_DRAGON_SCALE, Element::YETI_WATER, Element::GLACIER_ICE);
    pub const ELVEN_BREAD: FurnaceRecipe = FurnaceRecipe::new(Element::MAGMA_PEPPER, Element::BREAD_DOUGH, Element::ELVEN_BREAD);
    pub const ELVEN_TOAST: FurnaceRecipe = FurnaceRecipe::new(Element::MAGMA_PEPPER, Element::ELVEN_BREAD, Element::ELVEN_TOAST);
    pub const SCRAMBLED_EGG: FurnaceRecipe = FurnaceRecipe::new(Element::MAGMA_PEPPER, Element::GRIFFON_EGG, Element::SCRAMBLED_EGG);
    pub const BACON: FurnaceRecipe = FurnaceRecipe::new(Element::MAGMA_PEPPER, Element::RAW_BACON, Element::BACON);
    pub const DRIED_SEAWEED: FurnaceRecipe = FurnaceRecipe::new(Element::MAGMA_PEPPER, Element::SIREN_SEAWEED, Element::DRIED_SEAWEED);
    pub const BOILING_WATER: FurnaceRecipe = FurnaceRecipe::new(Element::MAGMA_PEPPER, Element::YETI_WATER, Element::BOILING_WATER);

    //pub const TEST: FurnaceRecipe = FurnaceRecipe::new(Element::FIRE_PEPPER, Element::YETI_WATER, Element::LEGEND_DAIRY);

    pub const RECIPES: [FurnaceRecipe; 7] = [
        FurnaceRecipe::GLACIER_ICE,
        FurnaceRecipe::ELVEN_BREAD,
        FurnaceRecipe::ELVEN_TOAST,
        FurnaceRecipe::SCRAMBLED_EGG,
        FurnaceRecipe::BACON,
        FurnaceRecipe::DRIED_SEAWEED,
        FurnaceRecipe::BOILING_WATER
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