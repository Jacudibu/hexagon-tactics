use bevy::utils::HashMap;

pub type PropId = usize;
pub struct PropDefinition {
    pub id: PropId,
    pub name: String,
}

pub const DEBUG_PROP_CUBE: PropId = 1;

impl PropDefinition {
    pub fn mock_data() -> HashMap<PropId, PropDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_PROP_CUBE,
            PropDefinition {
                id: DEBUG_PROP_CUBE,
                name: "Cube".to_string(),
            },
        );

        result
    }
}
