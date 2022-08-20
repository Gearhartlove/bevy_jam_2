use std::hash::{Hash, Hasher};
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::AppState;
use crate::element::ElementData;
use crate::furnace::FurnaceRecipe;
use crate::mixer::MixerRecipe;
use crate::slicer::SlicerRecipe;

//==================================================================================================
//                          Mixer Recipe Identifier
//==================================================================================================
#[derive(Eq, Debug)]
pub struct MixerRecipeIden {
    item_a : String,
    item_b : String
}

impl MixerRecipeIden {
    pub fn new(item_a : &str, item_b : &str) -> Self {
        MixerRecipeIden {
            item_a : item_a.to_string(),
            item_b : item_b.to_string(),
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
        let mut chars : Vec<char> = combined.chars().collect();
        chars.sort();
        chars.hash(state);
    }
}

//==================================================================================================
//                          Furnace Recipe Identifier
//==================================================================================================

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct FurnaceRecipeIden {
    fuel : String,
    object : String
}

impl FurnaceRecipeIden {
    pub fn new(fuel : &str, object : &str) -> Self {
        FurnaceRecipeIden {
            fuel : fuel.to_string(),
            object : object.to_string()
        }
    }
}

//==================================================================================================
//                          Registry
//==================================================================================================

#[derive(Default)]
pub struct Registry {
    pub element_registry: HashMap<String, ElementData>,
    pub mixer_recipe_registry : HashMap<MixerRecipeIden, MixerRecipe>,
    pub furnace_recipe_registry : HashMap<FurnaceRecipeIden, FurnaceRecipe>,
    pub slicer_recipe_registry : HashMap<String, SlicerRecipe>
}

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Registry::default())
        .add_startup_system(setup_elements)
        .add_startup_system(setup_recipes.after(setup_elements));
    }
}

fn setup_elements(mut registry : ResMut<Registry>) {
    let element_data = ElementData::load_from_dir("./assets/elements");
    for e in element_data {
        registry.element_registry.insert(e.id.clone(), e);
    }
    println!("Registry : {:?}", registry.element_registry);
    //state.set(AppState::Element_Setup).unwrap();
}

fn setup_recipes(mut registry : ResMut<Registry>) {
    let mixer_recipes = MixerRecipe::load_from_dir("./assets/recipes/mixer");
    for mr in mixer_recipes {
        if registry.element_registry.contains_key(&mr.first) && registry.element_registry.contains_key(&mr.second) && registry.element_registry.contains_key(&mr.result) {
            registry.mixer_recipe_registry.insert(MixerRecipeIden::new(mr.first.as_str(), mr.second.as_str()), mr);
        } else {
            warn!("Recipe '{}' was rejected: Input or output element not registered.", mr.id)
        }
    }

    let furnace_recipes = FurnaceRecipe::load_from_dir("./assets/recipes/furnace");
    for fr in furnace_recipes {
        if registry.element_registry.contains_key(&fr.fuel) && registry.element_registry.contains_key(&fr.object) && registry.element_registry.contains_key(&fr.result) {
            registry.furnace_recipe_registry.insert(FurnaceRecipeIden::new(fr.fuel.as_str(), fr.object.as_str()), fr);
        } else {
            warn!("Recipe '{}' was rejected: Input or output element not registered.", fr.id)
        }
    }

    let slicer_recipes = SlicerRecipe::load_from_dir("./assets/recipes/slicer/");
    for sr in slicer_recipes {
        if registry.element_registry.contains_key(&sr.object) && registry.element_registry.contains_key(&sr.result) {
            registry.slicer_recipe_registry.insert(sr.object.clone(), sr);
        } else {
            warn!("Recipe '{}' was rejected: Input or output element not registered.", sr.id)
        }
    }

    println!("Mixer Recipes : {:?}", registry.mixer_recipe_registry);
    println!("Furnace Recipes : {:?}", registry.furnace_recipe_registry);
    println!("Slicer Recipes : {:?}", registry.slicer_recipe_registry);
    //state.set(AppState::Game);
}

