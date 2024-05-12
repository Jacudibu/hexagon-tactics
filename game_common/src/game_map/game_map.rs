use crate::combat_data::CombatData;
use crate::combat_unit::CombatUnit;
use crate::game_map::field_of_movement_with_edge_detection::field_of_movement_with_edge_detection;
use crate::game_map::tile_data::TileData;
use crate::game_map::tile_surface::TileSurface;
use crate::game_map::versioned_map_data::VersionedMapData;
use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Resource, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameMap {
    pub radius: u32,
    pub tiles: HashMap<Hex, TileData>,
}

impl GameMap {
    #[must_use]
    pub fn new(radius: u32) -> Self {
        let mut tiles = HashMap::new();
        for hex in hexx::shapes::hexagon(Hex::ORIGIN, radius) {
            tiles.insert(
                hex,
                TileData {
                    height: 1,
                    surface: TileSurface::Grass,
                    fluid: None,
                },
            );
        }

        GameMap { radius, tiles }
    }

    pub fn write_to_disk(&self, path: &str) {
        // This clone is highly suboptimal.
        let _ = VersionedMapData::V1(self.clone()).write_to_disk(path);
    }

    pub fn load_from_file(path: &str) -> Result<Self, ()> {
        VersionedMapData::load_from_file(path)
    }

    #[must_use]
    pub fn field_of_movement(&self, unit: &CombatUnit, combat_data: &CombatData) -> Vec<Hex> {
        let unit_turn = combat_data.current_turn.as_unit_turn().unwrap();
        let mut result = field_of_movement_with_edge_detection(
            unit.position,
            unit_turn.remaining_movement.into(),
            |from, to| self.calculate_path_costs(unit, &combat_data, &from, &to),
        );

        // Since we can walk through our own units, remove tiles which are already occupied
        result.retain(|x| combat_data.unit_positions.get(x).is_none());
        result
    }

    #[must_use]
    pub fn calculate_path(&self, combat_data: &CombatData, coordinate: Hex) -> Option<Vec<Hex>> {
        let unit_turn = combat_data.current_turn.as_unit_turn().unwrap();
        let unit = combat_data.units.get(&unit_turn.unit_id).unwrap();
        hexx::algorithms::a_star(unit.position, coordinate, |from, to| {
            self.calculate_path_costs(unit, &combat_data, &from, &to)
        })
    }

    #[must_use]
    pub(crate) fn calculate_path_costs(
        &self,
        unit: &CombatUnit,
        combat_data: &CombatData,
        from: &Hex,
        to: &Hex,
    ) -> Option<u32> {
        let from_tile = &self.tiles[from];
        let to_tile = self.tiles.get(to)?;

        if let Some(unit_on_tile) = combat_data.unit_positions.get(to) {
            let unit_on_tile = &combat_data.units[unit_on_tile];
            if unit_on_tile.owner != unit.owner {
                return None;
            }
        }

        if to_tile.height == 0 {
            return None;
        }

        if let Some(fluid) = &to_tile.fluid {
            if fluid.height > 1.0 {
                return None;
            }
        }

        if from_tile.height.abs_diff(to_tile.height) > unit.stats_after_buffs.jump {
            return None;
        }

        Some(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::game_map::*;
    use crate::unit_stats::UnitStats;
    use assertables::{
        assert_contains, assert_contains_as_result, assert_not_contains,
        assert_not_contains_as_result,
    };

    use crate::combat_data::CombatData;
    use crate::combat_unit::CombatUnit;
    use crate::game_map::game_map::GameMap;
    use hexx::Hex;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn saving_and_loading() {
        let mut map = GameMap::new(3);
        let hex_with_different_height = Hex::new(-1, 0);
        let hex_with_different_surface = Hex::new(1, 0);

        map.tiles
            .get_mut(&hex_with_different_height)
            .unwrap()
            .height = 4;
        map.tiles
            .get_mut(&hex_with_different_surface)
            .unwrap()
            .surface = TileSurface::Stone;

        let dir = TempDir::new().unwrap();
        let bla = dir.path().join("saving_and_loading");
        let path = bla.to_str().unwrap();

        // Round-trip: Load -> Save -> Load
        map.write_to_disk(path);
        let loaded_map = GameMap::load_from_file(path).unwrap();
        assert_eq!(loaded_map, map);

        // Round-trip: Save -> Load -> Save
        let bla = dir.path().join("saving_and_loading");
        let path2 = bla.to_str().unwrap();
        loaded_map.write_to_disk(path);

        assert_eq!(
            fs::read_to_string(path2).unwrap(),
            fs::read_to_string(path).unwrap()
        );
    }

    #[test]
    fn field_of_movement() {
        let mut map = GameMap::new(3);
        let unit_pos = Hex::ZERO;
        let friendly_unit_pos = Hex::new(0, -1);
        let hostile_unit_pos = Hex::new(1, -1);

        let slightly_raised_hex = Hex::new(1, 0);
        let too_high_hex = Hex::new(-1, 0);
        let zero_height_tile = Hex::new(0, 1);

        map.tiles.get_mut(&zero_height_tile).unwrap().height = 0;
        map.tiles.get_mut(&slightly_raised_hex).unwrap().height = 2;
        map.tiles.get_mut(&too_high_hex).unwrap().height = 3;

        let mut combat_data = CombatData::create_mock().with_units(vec![
            CombatUnit::create_mock(1, 1)
                .with_position(unit_pos)
                .with_stats(UnitStats::create_mock().with_movement(1).with_jump(1)),
            CombatUnit::create_mock(2, 1).with_position(friendly_unit_pos),
            CombatUnit::create_mock(3, 2).with_position(hostile_unit_pos),
        ]);

        combat_data.start_unit_turn(1);

        let result = map.field_of_movement(&combat_data.units[&1], &combat_data);

        assert_not_contains!(
            result,
            &unit_pos,
            "Result should never contain the tile of the current unit"
        );

        assert_eq!(6 - 4, result.len());
        assert_contains!(result, &slightly_raised_hex);
        assert_not_contains!(result, &too_high_hex);
        assert_not_contains!(result, &zero_height_tile);
        assert_not_contains!(result, &friendly_unit_pos);
        assert_not_contains!(result, &hostile_unit_pos);
    }
}
