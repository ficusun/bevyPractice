use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod components;
pub mod events;
mod systems;
mod resources;

use systems::*;
use events::*;
use resources::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCharacter>()
        .insert_resource(DefaultState{ character_size: 50., health: 100., speed: 100., skill_cooldown_time: 2.5 })
        .add_systems(Update, character_spawner);
    }
}