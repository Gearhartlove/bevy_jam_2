use std::fs;
use std::path::Path;
use bevy::prelude::Res;
use scan_dir::ScanDir;
use serde::{Serialize, Deserialize};
use crate::registry::{FurnaceRecipeIden, Registry};

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Default)]
pub struct FurnaceRecipe {
    pub fuel : String,
    pub object : String,
    pub result : String,
    pub id : String
}

pub fn get_result(fuel : String, object : String, registry : &Res<Registry>) -> Option<String> {
    let iden = FurnaceRecipeIden::new(fuel.as_str(), object.as_str());
    if let Some(fr)  = registry.furnace_recipe_registry.get(&iden) {
        Some(fr.result.clone())
    } else {
        None
    }
}

impl FurnaceRecipe {
    pub fn load_from_dir(dir : &str) -> Vec<FurnaceRecipe> {
        ScanDir::files().read(dir, |iter| {
            let data : Vec<FurnaceRecipe> = iter
                .filter(|(_, name)| name.ends_with(".json"))
                .map(|(entry, _)| FurnaceRecipe::load_from_path(entry.path().as_path()))
                .filter(|element| element.is_some())
                .map(|element| element.unwrap())
                .collect();
            data
        }).unwrap()
    }

    pub fn load_from_path(path : &Path) -> Option<FurnaceRecipe> {
        let result = fs::read_to_string(path);
        if let Ok(json) = result {
            let data = serde_json::from_str::<FurnaceRecipe>(json.as_str());
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