use crate::load::HighlightMaterials;
use crate::map::highlights::HighlightedTiles;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Component, Resource};
use hexx::Hex;

#[derive(Component, Default)]
pub struct ActiveUnitHighlightMarker;

#[derive(Debug, Resource)]
pub struct ActiveUnitHighlights {
    pub tile: Hex,
}

impl HighlightedTiles for ActiveUnitHighlights {
    fn tiles(&self) -> impl Iterator<Item = &Hex> {
        std::iter::once(&self.tile)
    }

    fn material(materials: &HighlightMaterials) -> Handle<StandardMaterial> {
        materials.active_unit.clone()
    }

    fn name<'a>() -> &'a str {
        "Active Unit"
    }

    fn extra_height() -> f32 {
        0.0065
    }
}
