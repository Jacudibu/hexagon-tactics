pub mod attack_highlights;
mod cursor_highlights;
pub mod highlight_plugin;
pub mod range_highlights;

use crate::load::HighlightMaterials;
use bevy::asset::Handle;
use bevy::pbr::StandardMaterial;
use hexx::Hex;

trait HighlightedTiles {
    fn tiles(&self) -> impl Iterator<Item = &Hex>;

    fn material(materials: &HighlightMaterials) -> Handle<StandardMaterial>;

    fn name<'a>() -> &'a str;

    /// Vertical to the Hexagon Mesh.
    /// Used for layering highlights which aren't mutually exclusive to avoid z-fighting.
    fn extra_height() -> f32 {
        0.005
    }
}
