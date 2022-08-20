use std::fs;
use std::path::Path;
use bevy::prelude::Res;
use scan_dir::ScanDir;
use serde::{Serialize, Deserialize};
use crate::MixerRecipeIden;
use crate::registry::{FurnaceRecipeIden, Registry};

#[derive(PartialEq, Serialize, Deserialize, Default, Debug, Clone)]
pub struct MixerRecipe {
    pub first : String,
    pub second : String,
    pub result: String,
    pub id : String
}

pub fn get_result(element_a : String, element_b : String, registry : &Res<Registry>) -> Option<String> {
    let iden = MixerRecipeIden::new(element_a.as_str(), element_b.as_str());
    if let Some(fr)  = registry.mixer_recipe_registry.get(&iden) {
        Some(fr.result.clone())
    } else {
        None
    }
}

impl MixerRecipe {
    pub fn load_from_dir(dir : &str) -> Vec<MixerRecipe> {
        ScanDir::files().read(dir, |iter| {
            let data : Vec<MixerRecipe> = iter
                .filter(|(_, name)| name.ends_with(".json"))
                .map(|(entry, _)| MixerRecipe::load_from_path(entry.path().as_path()))
                .filter(|element| element.is_some())
                .map(|element| element.unwrap())
                .collect();
            data
        }).unwrap()
    }

    pub fn load_from_path(path : &Path) -> Option<MixerRecipe> {
        let result = fs::read_to_string(path);
        if let Ok(json) = result {
            let data = serde_json::from_str::<MixerRecipe>(json.as_str());
            if let Ok(data) = data {
                Some(data)
            } else {
                None
            }
        } else {
            None
        }
    }
}