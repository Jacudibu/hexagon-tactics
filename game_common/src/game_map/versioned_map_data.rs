use crate::game_map::game_map::GameMap;
use bevy::log::error;
use ron::de::from_reader;
use ron::to_string;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;

/// Enum to keep track of different Map Data Versions. Exclusively used for Saving and loading maps,
/// once that's done, everything else should just use the underlying GameMap of the latest version.
#[derive(Serialize, Deserialize)]
pub enum VersionedMapData {
    V1(GameMap),
}

impl VersionedMapData {
    pub fn write_to_disk(&self, path: &str) -> Result<(), ()> {
        // let config = PrettyConfig::new();
        match to_string(self) {
            Ok(result) => match fs::write(path, result) {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("Encountered Error when writing map data to disk: {:?}", e);
                    Err(())
                }
            },
            Err(e) => {
                error!("Encountered Error when serializing map data: {:?}", e);
                Err(())
            }
        }
    }

    pub fn load_from_file(path: &str) -> Result<GameMap, ()> {
        match File::open(path) {
            Ok(file) => {
                match from_reader::<File, Self>(file) {
                    Ok(data) => match data {
                        VersionedMapData::V1(map) => Ok(map),
                        // Older versions would call a .migrate method on them which would upgrade them to VX+1.
                    },
                    Err(e) => {
                        error!("Was unable to parse file. Error: {:?}", e);
                        Err(())
                    }
                }
            }
            Err(e) => {
                error!("Was unable to open file. Error: {:?}", e);
                Err(())
            }
        }
    }
}
