use std::fs;
use std::path::Path;
use scan_dir::ScanDir;
use serde::{Serialize, Deserialize};
use crate::mixer::MixerRecipe;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct SlicerRecipe {
    pub object : String,
    pub result : String,
    pub id : String
}

impl SlicerRecipe {
    pub fn load_from_dir(dir : &str) -> Vec<SlicerRecipe> {
        ScanDir::files().read(dir, |iter| {
            let data : Vec<SlicerRecipe> = iter
                .filter(|(_, name)| name.ends_with(".json"))
                .map(|(entry, _)| SlicerRecipe::load_from_path(entry.path().as_path()))
                .filter(|element| element.is_some())
                .map(|element| element.unwrap())
                .collect();
            data
        }).unwrap()
    }

    pub fn load_from_path(path : &Path) -> Option<SlicerRecipe> {
        let result = fs::read_to_string(path);
        if let Ok(json) = result {
            let data = serde_json::from_str::<SlicerRecipe>(json.as_str());
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