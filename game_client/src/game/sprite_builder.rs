use crate::load::CharacterSprites;
use crate::map::map_utils;
use bevy::math::Vec2;
use bevy::pbr::AlphaMode;
use bevy::prelude::{default, Handle, Image, Transform};
use bevy_sprite3d::{Sprite3d, Sprite3dBundle, Sprite3dParams};
use game_common::combat_unit::{CombatUnit, CombatUnitKind};
use game_common::game_map::GameMap;

pub fn build_unit_sprite(
    unit: &CombatUnit,
    character_sprites: &CharacterSprites,
    map: &GameMap,
    sprite_params: &mut Sprite3dParams,
) -> Sprite3dBundle {
    let image = match unit.kind {
        CombatUnitKind::Humanoid(_) => character_sprites.test.clone(),
        CombatUnitKind::Monster(_) => character_sprites.test_monster.clone(),
    };

    build(unit, image, map, sprite_params)
}

// TODO: This should not require a complete sprite rebuild and just swap out the texture
pub fn build_dead_unit_sprite(
    unit: &CombatUnit,
    character_sprites: &CharacterSprites,
    map: &GameMap,
    sprite_params: &mut Sprite3dParams,
) -> Sprite3dBundle {
    let image = match unit.kind {
        CombatUnitKind::Humanoid(_) => character_sprites.test_dead.clone(),
        CombatUnitKind::Monster(_) => character_sprites.test_monster_dead.clone(),
    };

    build(unit, image, map, sprite_params)
}

fn build(
    unit: &CombatUnit,
    image: Handle<Image>,
    map: &GameMap,
    mut sprite_params: &mut Sprite3dParams,
) -> Sprite3dBundle {
    Sprite3d {
        image,
        pixels_per_metre: 16.0,
        alpha_mode: AlphaMode::Mask(0.1),
        unlit: false,
        double_sided: true, // required for shadows
        pivot: Some(Vec2::new(0.5, 0.0)),
        transform: Transform::from_translation(map_utils::unit_position_on_hexagon(
            unit.position,
            &map,
        )),
        ..default()
    }
    .bundle(&mut sprite_params)
}
