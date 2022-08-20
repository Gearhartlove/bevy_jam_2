use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::element::ElementData;

pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Registry::default())
            .add_startup_system(load_all_elements);
    }
}

#[derive(Default)]
pub struct Registry {
    item_registry : HashMap<String, ElementData>
}

pub fn load_all_elements(mut registry : ResMut<Registry>) {
    let element_data = ElementData::load_from_dir("./assets/elements");
    for e in element_data {
        registry.item_registry.insert(e.id.clone(), e);
    }
    println!("Registry : {:?}", registry.item_registry)
}

