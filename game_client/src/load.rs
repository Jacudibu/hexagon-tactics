use crate::map::METERS_PER_TILE_HEIGHT_UNIT;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::texture::{
    ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
};
use bevy::utils::HashMap;
use game_common::game_data::GameData;
use game_common::game_map;
use game_common::game_map::{FluidKind, TileData, TileSurface, HEX_LAYOUT};
use hexx::{ColumnMeshBuilder, HexLayout, MeshInfo, UVOptions};

pub struct LoadPlugin;
impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_materials, load_meshes, load_sprites));
    }
}

#[derive(Debug, Resource)]
pub struct HexagonMaterials {
    pub top: HexagonMaterialsForSideOrTop,
    pub sides: HexagonMaterialsForSideOrTop,
    pub fluid: HexagonMaterialsForFluid,
}

#[derive(Debug)]
pub struct HexagonMaterialsForSideOrTop {
    pub invisible: Handle<StandardMaterial>,
    pub grass: Handle<StandardMaterial>,
    pub stone: Handle<StandardMaterial>,
    pub sand: Handle<StandardMaterial>,
    pub earth: Handle<StandardMaterial>,
}

impl HexagonMaterialsForSideOrTop {
    #[must_use]
    pub fn surface_material(&self, tile_data: &TileData) -> Handle<StandardMaterial> {
        if tile_data.height == 0 {
            self.invisible.clone()
        } else {
            match tile_data.surface {
                TileSurface::Grass => self.grass.clone(),
                TileSurface::Stone => self.stone.clone(),
                TileSurface::Sand => self.sand.clone(),
                TileSurface::Earth => self.earth.clone(),
            }
        }
    }
}

#[derive(Debug)]
pub struct HexagonMaterialsForFluid {
    pub water: Handle<StandardMaterial>,
}

impl HexagonMaterialsForFluid {
    #[must_use]
    pub fn surface_material(&self, fluid: &FluidKind) -> Handle<StandardMaterial> {
        match fluid {
            FluidKind::Water => self.water.clone(),
        }
    }
}

#[derive(Debug, Resource)]
pub struct HighlightMaterials {
    pub cursor: Handle<StandardMaterial>,
    pub range: Handle<StandardMaterial>,
    pub attack: Handle<StandardMaterial>,
    pub active_unit: Handle<StandardMaterial>,
}

#[derive(Debug, Resource)]
pub struct HexagonMeshes {
    pub flat: Handle<Mesh>,
    pub columns: HashMap<u8, Handle<Mesh>>,
}

#[derive(Debug, Resource)]
pub struct CharacterSprites {
    pub test: Handle<Image>,
    pub test_dead: Handle<Image>,
}

pub fn load_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let flat_mesh = generate_hexagon_flat_mesh(&HEX_LAYOUT);
    let flat = meshes.add(flat_mesh);

    let mut columns = HashMap::new();
    for height in 0..=game_map::MAX_HEIGHT {
        let mesh = generate_hexagonal_column_mesh(
            &HEX_LAYOUT,
            height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
        );
        let handle = meshes.add(mesh);
        columns.insert(height, handle);
    }

    commands.insert_resource(HexagonMeshes { flat, columns })
}

fn generate_hexagon_flat_mesh(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = hexx::mesh::PlaneMeshBuilder::new(hex_layout).build();
    generate_mesh(mesh_info)
}

fn generate_hexagonal_column_mesh(hex_layout: &HexLayout, height: f32) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, height)
        .without_bottom_face()
        .without_top_face()
        .center_aligned()
        .with_sides_uv_options(UVOptions {
            flip: BVec2 { x: true, y: true },
            rect: hexx::Rect {
                min: Vec2::ZERO,
                max: Vec2::new(1.0, height),
            },
            ..default()
        })
        .build();

    generate_mesh(mesh_info)
}

fn generate_mesh(mesh_info: MeshInfo) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

pub fn load_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let grass_top = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/grass_top.png")),
        reflectance: 0.2,
        perceptual_roughness: 0.7,
        ..default()
    });
    let grass_side = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load_with_settings(
            "textures/grass_side.png",
            load_image_with_repeated_address_mode,
        )),
        reflectance: 0.2,
        perceptual_roughness: 0.7,
        ..default()
    });

    let water = materials.add(StandardMaterial {
        base_color: Color::rgba(0.0, 0.5, 1.0, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.insert_resource(HexagonMaterials {
        top: {
            HexagonMaterialsForSideOrTop {
                invisible: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                grass: grass_top,
                stone: generate_generic_top_mat(Color::GRAY, &asset_server, &mut materials),
                sand: generate_generic_top_mat(Color::BEIGE, &asset_server, &mut materials),
                earth: generate_generic_top_mat(Color::TOMATO, &asset_server, &mut materials),
            }
        },
        sides: {
            HexagonMaterialsForSideOrTop {
                invisible: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                grass: grass_side,
                stone: generate_generic_side_mat(Color::GRAY, &asset_server, &mut materials),
                sand: generate_generic_side_mat(Color::BEIGE, &asset_server, &mut materials),
                earth: generate_generic_side_mat(Color::TOMATO, &asset_server, &mut materials),
            }
        },
        fluid: { HexagonMaterialsForFluid { water } },
    });

    commands.insert_resource(HighlightMaterials {
        cursor: materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            base_color_texture: Some(asset_server.load("textures/tile_cursor.png")),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        range: materials.add(StandardMaterial {
            base_color: Color::rgba(0.0, 1.0, 0.0, 1.0),
            base_color_texture: Some(asset_server.load("textures/tile_cursor_range.png")),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        attack: materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 0.0, 0.0, 1.0),
            base_color_texture: Some(asset_server.load("textures/tile_cursor_attack.png")),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        active_unit: materials.add(StandardMaterial {
            base_color: Color::rgba(0.0, 0.0, 1.0, 1.0),
            base_color_texture: Some(asset_server.load("textures/tile_cursor_active_unit.png")),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    });
}

fn load_image_with_repeated_address_mode(settings: &mut ImageLoaderSettings) {
    let sampler = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..Default::default()
    };

    settings.sampler = ImageSampler::Descriptor(sampler);
}

fn generate_generic_top_mat(
    color: Color,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        base_color_texture: Some(asset_server.load("textures/white_with_highlights_32px.png")),
        reflectance: 0.2,
        perceptual_roughness: 0.7,
        ..default()
    })
}

fn generate_generic_side_mat(
    color: Color,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        base_color_texture: Some(asset_server.load_with_settings(
            "textures/white_with_highlights_16px.png",
            load_image_with_repeated_address_mode,
        )),
        reflectance: 0.2,
        perceptual_roughness: 0.7,
        ..default()
    })
}

pub fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let test = asset_server.load("sprites/test_character.png");
    let test_dead = asset_server.load("sprites/test_character_dead.png");

    commands.insert_resource(CharacterSprites { test, test_dead });
    commands.insert_resource(GameData::load());
}
