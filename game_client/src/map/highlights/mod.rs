pub mod attack_highlights;
mod cursor_highlights;
pub mod highlight_plugin;
pub mod range_highlights;

use crate::load::CursorMaterials;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use hexx::Hex;

trait HighlightedTiles {
    fn tiles(&self) -> &Vec<Hex>;
    fn material(materials: &CursorMaterials) -> Handle<StandardMaterial>;
    fn name<'a>() -> &'a str;
}
