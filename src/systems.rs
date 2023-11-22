use crate::components::{Health, Speed, PlayerInput};
use crate::res::{CharacterDefault, WorldSize};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::{thread_rng, Rng};

pub fn spawn(
    mut commands: Commands,
    def_state: Res<CharacterDefault>,
    world_size: Res<WorldSize>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = thread_rng();

    let x: f32 = rng.gen_range(0. ..world_size.range);
    let y: f32 = rng.gen_range(0. ..world_size.range);

    let mut entity_commands = commands.spawn(MaterialMesh2dBundle {
        mesh: meshes // Vec2::new(size_of_quad, size_of_quad)
            .add(shape::Circle::new(def_state.character_size / 2.).into()) //
            .into(),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        transform: Transform::from_translation(Vec3::new(x, y, 1.)),
        ..default()
    });

    entity_commands.insert(Health { amount: 100.0 });
    entity_commands.insert(PlayerInput::default());
    entity_commands.insert(Speed{ val: 100f32});
}

pub fn movement (mut players: Query<(&mut Transform, &Speed, &PlayerInput)>, time: Res<Time>, world_size: Res<WorldSize>) {
    players.for_each_mut(|(mut transform, speed, player_input)| {
        
        let new_pos = transform.translation + player_input.0.normalize_or_zero() * speed.val * time.delta_seconds();
        if new_pos.x > world_size.range || new_pos.x < 0. {
            return
        }

        if new_pos.y > world_size.range || new_pos.y < 0. {
            return
        }
        
        transform.translation = new_pos;

    });
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_query: Query<&mut PlayerInput>,
) {

    match input_query.get_single_mut() {
        Ok(mut input) => {

            let mut vel = Vec3::default();

            if keyboard_input.pressed(KeyCode::W) {
                vel.y += 1.;
            }

            if keyboard_input.pressed(KeyCode::A) {
                vel.x -= 1.;
            }

            if keyboard_input.pressed(KeyCode::D) {
                vel.x += 1.;
            }

            if keyboard_input.pressed(KeyCode::S) {
                vel.y -= 1.;
            }
            
            input.0 = vel; 
            
        },
        Err(err) => {println!("{err:?}")}
    };
}
