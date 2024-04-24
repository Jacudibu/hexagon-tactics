use crate::load::CursorMaterials;
use crate::map::highlights::HighlightedTiles;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Component, Resource};
use hexx::Hex;

#[derive(Component, Default)]
pub struct RangeHighlightMarker;

#[derive(Debug, Resource)]
pub struct RangeHighlights {
    pub tiles: Vec<Hex>,
}

impl HighlightedTiles for RangeHighlights {
    fn tiles(&self) -> &Vec<Hex> {
        &self.tiles
    }

    fn material(materials: &CursorMaterials) -> Handle<StandardMaterial> {
        materials.range_highlight.clone()
    }
}
