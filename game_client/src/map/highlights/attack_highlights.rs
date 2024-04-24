use crate::load::CursorMaterials;
use crate::map::highlights::HighlightedTiles;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Component, Resource};
use hexx::Hex;

#[derive(Component, Default)]
pub struct AttackHighlightMarker;

#[derive(Debug, Resource)]
pub struct AttackHighlights {
    pub tiles: Vec<Hex>,
}

impl HighlightedTiles for AttackHighlights {
    fn tiles(&self) -> &Vec<Hex> {
        &self.tiles
    }

    fn material(materials: &CursorMaterials) -> Handle<StandardMaterial> {
        materials.attack_highlight.clone()
    }

    fn name<'a>() -> &'a str {
        "Attack"
    }
}
