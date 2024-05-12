use crate::game_data::skill::SkillDefinition;
use crate::unit::CombatUnit;
use crate::validation::validation_error::ValidationError;

pub fn validate_unit_has_enough_resources_to_use_skill(
    unit: &CombatUnit,
    skill: &SkillDefinition,
) -> Result<(), ValidationError> {
    if unit.mp < skill.mp_costs {
        Err(ValidationError::new("Not enough MP to use this skill!"))
    } else {
        Ok(())
    }
}
