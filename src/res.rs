use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WorldSize {
    pub range: f32,
    pub world_y: f32,
    pub world_x: f32,
}

#[derive(Resource, Default)]
pub struct CharacterDefault {
    pub character_size: f32,
    pub health: f32,
}