use bevy::prelude::*;

// .add_event::<SpawnCharacter>()
#[derive(Event)]
pub struct SpawnCharacter {
    pub is_player: bool
}