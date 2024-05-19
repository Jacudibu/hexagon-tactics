use crate::combat_unit::CombatUnit;
use crate::game_data::skill::{SkillDefinition, SkillId};
use crate::game_data::GameData;
use crate::validation::validation_error::ValidationError;

pub fn validate_unit_knows_skill<'a>(
    unit: &CombatUnit,
    skill_id: SkillId,
    game_data: &'a GameData,
) -> Result<&'a SkillDefinition, ValidationError> {
    match unit
        .all_available_skills(game_data)
        .iter()
        .find(|x| x == &&skill_id)
    {
        None => Err(ValidationError::new("This unit doesn't know that skill!")),
        Some(skill_id) => Ok(&game_data.skills[skill_id]),
    }
}
