use crate::load::HighlightMaterials;
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
    fn tiles(&self) -> impl Iterator<Item = &Hex> {
        self.tiles.iter()
    }

    fn material(materials: &HighlightMaterials) -> Handle<StandardMaterial> {
        materials.attack.clone()
    }

    fn name<'a>() -> &'a str {
        "Attack"
    }
}
