use crate::map_editor::map_editor_tool::MapEditorTool;
use bevy::log::warn;
use bevy::prelude::{KeyCode, MouseButton, Reflect};
use game_common::game_map::{FluidKind, TileSurface};
use leafwing_input_manager::axislike::DualAxis;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::prelude::InputKind;
use leafwing_input_manager::Actionlike;
use std::fmt::Formatter;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MapEditorAction {
    UseTool,
    RaiseTiles,
    LowerTiles,
    PaintGrass,
    PaintStone,
    PaintSand,
    PaintEarth,
    RaiseWater,
    LowerFluid,
    MouseMotion,
    MarkSpawnTile,
    RemoveSpawnTile,
    SpawnCubeProp,
}

impl std::fmt::Display for MapEditorAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapEditorAction::UseTool => write!(f, "Use Tool"),
            MapEditorAction::RaiseTiles => write!(f, "Raise Tiles"),
            MapEditorAction::LowerTiles => write!(f, "Lower Tiles"),
            MapEditorAction::PaintGrass => write!(f, "Paint Grass"),
            MapEditorAction::PaintStone => write!(f, "Paint Stone"),
            MapEditorAction::PaintSand => write!(f, "Paint Sand"),
            MapEditorAction::PaintEarth => write!(f, "Paint Earth"),
            MapEditorAction::RaiseWater => write!(f, "Raise Water"),
            MapEditorAction::LowerFluid => write!(f, "Lower Water"),
            MapEditorAction::MarkSpawnTile => write!(f, "Mark Spawn Tile"),
            MapEditorAction::RemoveSpawnTile => write!(f, "Remove Spawn Tile"),
            MapEditorAction::SpawnCubeProp => write!(f, "Spawn Cube Prop"),
            MapEditorAction::MouseMotion => {
                warn!(
                    "MapEditorAction::MouseMotion::Display was called. This should never happen?"
                );
                write!(f, "Mouse Motion")
            }
        }
    }
}

impl MapEditorAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::UseTool, InputKind::Mouse(MouseButton::Left));
        input_map.insert(Self::RaiseTiles, KeyCode::KeyR);
        input_map.insert(Self::LowerTiles, KeyCode::KeyF);
        input_map.insert(Self::PaintGrass, KeyCode::Digit1);
        input_map.insert(Self::PaintStone, KeyCode::Digit2);
        input_map.insert(Self::PaintSand, KeyCode::Digit3);
        input_map.insert(Self::PaintEarth, KeyCode::Digit4);
        input_map.insert(Self::RaiseWater, KeyCode::Digit5);
        input_map.insert(Self::LowerFluid, KeyCode::Digit6);
        input_map.insert(Self::MarkSpawnTile, KeyCode::KeyZ);
        input_map.insert(Self::RemoveSpawnTile, KeyCode::KeyX);
        input_map.insert(Self::SpawnCubeProp, KeyCode::KeyC);
        input_map.insert(Self::MouseMotion, DualAxis::mouse_motion());

        input_map
    }
}

#[rustfmt::skip]
pub const ACTION_TO_TOOL: [(MapEditorAction, MapEditorTool); 11] = [
    (MapEditorAction::RaiseTiles, MapEditorTool::RaiseTiles),
    (MapEditorAction::LowerTiles, MapEditorTool::LowerTiles),
    (MapEditorAction::PaintGrass, MapEditorTool::PaintSurface(TileSurface::Grass)),
    (MapEditorAction::PaintStone, MapEditorTool::PaintSurface(TileSurface::Stone)),
    (MapEditorAction::PaintSand,  MapEditorTool::PaintSurface(TileSurface::Sand)),
    (MapEditorAction::PaintEarth, MapEditorTool::PaintSurface(TileSurface::Earth)),
    (MapEditorAction::RaiseWater, MapEditorTool::RaiseFluid(FluidKind::Water)),
    (MapEditorAction::LowerFluid, MapEditorTool::LowerFluid),
    (MapEditorAction::MarkSpawnTile, MapEditorTool::MarkSpawnTile(1)),
    (MapEditorAction::RemoveSpawnTile, MapEditorTool::RemoveSpawnTile),
    (MapEditorAction::SpawnCubeProp, MapEditorTool::SpawnProp(1)),
];
