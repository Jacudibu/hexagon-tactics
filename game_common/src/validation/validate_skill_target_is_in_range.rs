use crate::skill::{Skill, SkillTargeting};
use crate::validation::validation_error::ValidationError;
use hexx::Hex;

pub fn validate_skill_target_is_in_range(
    skill: &Skill,
    origin: Hex,
    target: Hex,
) -> Result<(), ValidationError> {
    match &skill.targeting {
        SkillTargeting::UserPosition => Ok(()),
        SkillTargeting::MouseCursor(range) => {
            let distance = origin.unsigned_distance_to(target);
            if distance < range.min {
                Err(ValidationError::new("Target is too close!"))
            } else if distance > range.max {
                Err(ValidationError::new("Target is too far!"))
            } else {
                Ok(())
            }
        }
    }
}
