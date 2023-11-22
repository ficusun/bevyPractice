use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DefaultState {
    pub character_size: f32,
    pub health: f32,
    pub speed: f32,
    pub skill_cooldown_time: f32 // seconds
}