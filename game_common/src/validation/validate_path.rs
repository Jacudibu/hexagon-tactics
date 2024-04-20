use crate::combat_data::CombatData;
use crate::player::PlayerId;
use crate::validation::validation_error::ValidationError;
use hexx::Hex;

pub fn validate_path_for_current_unit(
    player_id: PlayerId,
    combat_data: &CombatData,
    path: &Vec<Hex>,
) -> Result<(), ValidationError> {
    Ok(())
}
