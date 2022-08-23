use bevy::prelude::Res;
use crate::MixerRecipeIden;
use crate::registry::{Registry};
use bevy::prelude::*;
use crate::element::Element;

#[derive(PartialEq, Default, Debug, Clone)]
pub struct MixerRecipe {
    pub first: Element,
    pub second: Element,
    pub result: Element,
}

impl MixerRecipe {
    pub const UTTER_ICE_CREAM: MixerRecipe = MixerRecipe::new(Element::SHAVED_ICE, Element::LEGEND_DAIRY, Element::UTTER_ICE_CREAM);

    pub const RECIPES: [MixerRecipe; 1] = [
        MixerRecipe::UTTER_ICE_CREAM,
    ];

    pub const fn new(first: Element, second: Element, result: Element) -> Self {
        Self {
            first,
            second,
            result,
        }
    }

    pub fn id(&self) -> String {
        let id = format!("{}_{}_{}", self.first.id, self.second.id, self.result.id);
        return id;
    }
}

pub fn get_result(element_a: Element, element_b: Element, registry: &Res<Registry>) -> Option<Element> {
    let iden = MixerRecipeIden::new(element_a, element_b);
    if let Some(mr) = registry.mixer_recipe_registry.get(&iden) {
        Some(mr.result.clone())
    } else {
        None
    }
}