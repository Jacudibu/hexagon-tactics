use crate::combat_data::CombatData;
use crate::combat_unit::UnitId;
use crate::game_map::GameMap;
use crate::validation::validation_error::ValidationError;
use hexx::Hex;

pub fn validate_unit_can_be_placed_on_tile<'a>(
    combat_data: &CombatData,
    unit_id: &UnitId,
    team: &u8,
    hex: &Hex,
    map: &GameMap,
) -> Result<(), ValidationError> {
    if combat_data.unit_positions.contains_key(hex) {
        return Err(ValidationError::new("Tile is already occupied"));
    }

    if combat_data.units.contains_key(unit_id) {
        return Err(ValidationError::new("Unit has already been placed!"));
    }

    let Some(tile) = map.tiles.get(hex) else {
        return Err(ValidationError::new(format!(
            "Invalid tile coordinates: {:?}",
            hex
        )));
    };

    match tile.can_unit_be_placed_here(team) {
        true => Ok(()),
        false => Err(ValidationError::new(format!(
            "Invalid tile coordinates: {:?}",
            hex
        ))),
    }
}
