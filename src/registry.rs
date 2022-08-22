use std::hash::{Hash, Hasher};
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::element::{Element};
use crate::furnace::{FurnaceRecipe};
use crate::mixer::MixerRecipe;
use crate::slicer::SlicerRecipe;

//==================================================================================================
//                          Mixer Recipe Identifier
//==================================================================================================
#[derive(Eq, Debug)]
pub struct MixerRecipeIden {
    item_a: Element,
    item_b: Element,
}

impl MixerRecipeIden {
    pub fn new(item_a: Element, item_b: Element) -> Self {
        MixerRecipeIden {
            item_a,
            item_b,
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
        let combined = format!("{}{}", self.item_a.id, self.item_b.id);
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
    pub furnace_recipe_registry: HashMap<FurnaceRecipeIden, FurnaceRecipe>,
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

fn setup_registry(_registry: &mut Registry) -> Registry {
    let mut _registry = Registry::default();

    // mixer recipe
    add_mixer_recipes_to_registry(&mut _registry);
    // furnace recipe
    add_furnace_recipes_to_registry(&mut _registry);
    // slicer recipe
    add_slicer_recipes_to_registry(&mut _registry);

    println!("Mixer Recipes : {:?}", _registry.mixer_recipe_registry);
    println!("Furnace Recipes : {:?}", _registry.furnace_recipe_registry);
    println!("Slicer Recipes : {:?}", _registry.slicer_recipe_registry);

    return _registry;
}

// FurnaceRecipe { fuel, object, result, id }
fn add_furnace_recipes_to_registry(registry: &mut Registry) {
    for fr in FurnaceRecipe::RECIPES {
        registry.furnace_recipe_registry.insert(FurnaceRecipeIden::new(fr.fuel.clone(), fr.object.clone()), fr.clone());
    }
}


// slicer { object, result, id }
fn add_slicer_recipes_to_registry(registry: &mut Registry) {
    for sr in SlicerRecipe::RECIPES {
        registry.slicer_recipe_registry.insert(sr.object.clone(), sr);
    }
}

// mixer { first, second, result, id }
// note: order of first and second does not matter
fn add_mixer_recipes_to_registry(registry: &mut Registry) {
    for mr in MixerRecipe::RECIPES {
        registry.mixer_recipe_registry.insert(MixerRecipeIden::new(mr.first.clone(), mr.second.clone()), mr.clone());
    }
}

