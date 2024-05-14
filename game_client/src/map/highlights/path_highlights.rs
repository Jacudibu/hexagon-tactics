use crate::load::HighlightMaterials;
use crate::map::highlights::HighlightedTiles;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Component, Resource};
use hexx::Hex;

#[derive(Component, Default)]
pub struct PathHighlightMarker;

#[derive(Debug, Resource)]
pub struct PathHighlights {
    pub tiles: Vec<Hex>,
}

impl HighlightedTiles for PathHighlights {
    fn tiles(&self) -> impl Iterator<Item = &Hex> {
        self.tiles.iter()
    }

    fn material(materials: &HighlightMaterials) -> Handle<StandardMaterial> {
        materials.path_dot.clone()
    }

    fn name<'a>() -> &'a str {
        "Path"
    }

    fn extra_height() -> f32 {
        0.0075
    }
}
