use std::{fs, io};
use std::path::Path;
use scan_dir::ScanDir;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ElementData {
    pub name : String,
    pub id : String,
    pub sprite : String,
    pub desc : String
}

impl ElementData {
    pub fn load_from_dir(dir : &str) -> Vec<ElementData> {
        ScanDir::files().read(dir, |iter| {
            let data : Vec<ElementData> = iter
                .filter(|(_, name)| name.ends_with(".json"))
                .map(|(entry, _)| ElementData::load_from_path(entry.path().as_path()))
                .filter(|element| element.is_some())
                .map(|element| element.unwrap())
                .collect();
            data
        }).unwrap()
    }

    pub fn load_from_path(path : &Path) -> Option<ElementData> {
        let result = fs::read_to_string(path);
        if let Ok(json) = result {
            let data = serde_json::from_str::<ElementData>(json.as_str());
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