use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Health {
    pub amount: f32,
}

#[derive(Component, Default)]
pub struct PlayerInput(pub Vec3);

#[derive(Component, Default)]
pub struct Speed {
    pub val: f32,
}