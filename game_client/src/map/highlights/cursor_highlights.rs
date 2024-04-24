use crate::load::CursorMaterials;
use crate::map::highlights::HighlightedTiles;
use crate::map::CursorOnTile;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::Component;
use hexx::Hex;

#[derive(Component, Default)]
pub struct CursorHighlightMarker;

impl HighlightedTiles for CursorOnTile {
    fn tiles(&self) -> &Vec<Hex> {
        &self.temp_hexes
    }

    fn material(materials: &CursorMaterials) -> Handle<StandardMaterial> {
        materials.default.clone()
    }

    fn name<'a>() -> &'a str {
        "Cursor"
    }
}
