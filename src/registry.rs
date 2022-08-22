use std::hash::{Hash, Hasher};
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::AppState;
use crate::element::{Element, ElementData};
use crate::furnace::{FurnaceData, FurnaceRecipe};
use crate::mixer::MixerRecipe;
use crate::slicer::SlicerRecipe;

//==================================================================================================
//                          Mixer Recipe Identifier
//==================================================================================================
#[derive(Eq, Debug)]
pub struct MixerRecipeIden {
    item_a: String,
    item_b: String,
}

impl MixerRecipeIden {
    pub fn new(item_a: &str, item_b: &str) -> Self {
        MixerRecipeIden {
            item_a: item_a.to_string(),
            item_b: item_b.to_string(),
        }
    }
}

impl PartialEq for MixerRecipeIden {
    fn eq(&self, other: &Self) -> bool {
        let case_a = self.item_a == other.item_a && self.item_b == other.item_b;
        let case_b = self.item_a == other.item_b && self.item_b == other.item_a;
        case_a || case_b
    }
}

impl Hash for MixerRecipeIden {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let combined = format!("{}{}", self.item_a, self.item_b);
        let mut chars: Vec<char> = combined.chars().collect();
        chars.sort();
        chars.hash(state);
    }
}

//==================================================================================================
//                          Furnace Recipe Identifier
//==================================================================================================

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct FurnaceRecipeIden {
    fuel: Element,
    object: Element,
}

impl FurnaceRecipeIden {
    pub fn new(fuel: Element, object: Element) -> Self {
        FurnaceRecipeIden {
            fuel,
            object,
        }
    }
}

//==================================================================================================
//                          Registry
//==================================================================================================

pub struct Registry {
    pub mixer_recipe_registry: HashMap<MixerRecipeIden, MixerRecipe>,
    pub furnace_recipe_registry: HashMap<FurnaceRecipeIden, FurnaceData>,
    pub slicer_recipe_registry: HashMap<Element, SlicerRecipe>,
}

impl Default for Registry {
    fn default() -> Self {
        let mut registry = Registry {
            mixer_recipe_registry: Default::default(),
            furnace_recipe_registry: Default::default(),
            slicer_recipe_registry: Default::default(),
        };
        setup_registry(&mut registry);
        return registry;
    }
}

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Registry>();
    }
}

fn setup_registry(registry: &mut Registry) -> Registry {
    let mut registry = Registry::default();

    // mixer recipe
    add_mixer_recipes_to_registry(&mut registry);
    // furnace recipe
    add_furnace_recipes_to_registry(&mut registry);
    // slicer recipe
    add_slicer_recipes_to_registry(&mut registry);

    println!("Mixer Recipes : {:?}", registry.mixer_recipe_registry);
    println!("Furnace Recipes : {:?}", registry.furnace_recipe_registry);
    println!("Slicer Recipes : {:?}", registry.slicer_recipe_registry);

    return registry;
}

// FurnaceRecipe { fuel, object, result, id }
fn add_furnace_recipes_to_registry(registry: &mut Registry) {
    for fr in FurnaceRecipe::RECIPES {
        registry.furnace_recipe_registry.insert(FurnaceRecipeIden::new(fr.data.fuel.clone(), fr.data.object.clone()), fr.data.clone());
    }
}


// slicer { object, result, id }
fn add_slicer_recipes_to_registry(registry: &mut Registry) {
    // let slicer_recipes = vec!(
    //     // Frost Bottle
    //     SlicerRecipe::new("Glacier Ice", "Shaved Ice")
    // );

    // for sr in slicer_recipes {
    //     if registry.element_registry.contains_key(&sr.object) && registry.element_registry.contains_key(&sr.result) {
    //         registry.slicer_recipe_registry.insert(sr.object.clone(), sr);
    //     } else {
    //         warn!("Recipe '{}' was rejected: Input or output element not registered.", sr.id)
    //     }
    // }
}

// mixer { first, second, result, id }
// note: order of first and second does not matter
fn add_mixer_recipes_to_registry(registry: &mut Registry) {
    // let mixer_recipes = vec!(
    //     MixerRecipe::new("Legend Dairy", "Shaved Ice", "Legend ice cream"),
    // );
}

