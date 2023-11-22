pub mod components;
pub mod res;
pub mod systems;

pub mod character;
pub mod controll;

use controll::ControllPlugin;
use character::CharacterPlugin;
use components::*;
use res::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldSize {
            range: 400.,
            world_y: 100.,
            world_x: 100.,
        })
        // .add_systems(Startup, setup)
        .insert_resource(CharacterDefault {
            character_size: 40.,
            health: 100.,
        })
        .add_plugins(CharacterPlugin)
        .add_plugins(ControllPlugin)
        //.add_systems(Startup, spawn)
        //.add_systems(Update, keyboard_input_system)
        //.add_systems(Update, movement)
        .run();
}

// fn setup(mut commands: Commands, mut test: ResMut<Events<SpawnCharacter>>,) {
//     commands.spawn(Camera2dBundle::default());

//     test.send(SpawnCharacter{ is_player: true });
// }