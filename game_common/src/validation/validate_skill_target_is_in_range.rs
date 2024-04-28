use crate::skill::{Skill, SkillShape};
use crate::validation::validation_error::ValidationError;
use hexx::Hex;

pub fn validate_skill_target_is_in_range(
    skill: &Skill,
    origin: Hex,
    target: Hex,
) -> Result<(), ValidationError> {
    if let SkillShape::Custom(custom) = &skill.shape {
        if custom.centered_around_user {
            return Ok(());
        }
    }

    let distance = origin.unsigned_distance_to(target);
    if distance < skill.range.min {
        Err(ValidationError::new("Target is too close!"))
    } else if distance > skill.range.max {
        Err(ValidationError::new("Target is too far!"))
    } else {
        Ok(())
    }
}
