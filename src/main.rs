mod registry;
mod element;

use bevy::prelude::*;
use crate::element::ElementData;
use crate::registry::RegistryPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RegistryPlugin)
        .run();
}
