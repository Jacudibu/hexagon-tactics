use crate::load::HighlightMaterials;
use crate::map::highlights::HighlightedTiles;
use crate::map::CursorOnTile;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::Component;
use hexx::Hex;

#[derive(Component, Default)]
pub struct CursorHighlightMarker;

impl HighlightedTiles for CursorOnTile {
    fn tiles(&self) -> impl Iterator<Item = &Hex> {
        std::iter::once(&self.hex)
    }

    fn material(materials: &HighlightMaterials) -> Handle<StandardMaterial> {
        materials.cursor.clone()
    }

    fn name<'a>() -> &'a str {
        "Cursor"
    }

    fn extra_height() -> f32 {
        0.01
    }
}
