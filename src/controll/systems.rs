use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::character::{components::IsPlayer, events::SpawnCharacter};

pub fn setup(mut commands: Commands, mut test: ResMut<Events<SpawnCharacter>>) {
    commands.spawn(Camera2dBundle::default());

    test.send(SpawnCharacter { is_player: true });
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    mut players_transform: Query<&mut Transform, With<IsPlayer>>,
) {
    let mut window = windows.single_mut();

    let mut player: Option<Vec2> = players_transform
        .get_single_mut()
        .map_or_else(None, |p| Some(p.translation.xy())); //single_mut()

    match (
        players_transform
            .get_single_mut()
            .map_or(None, |p| Some(p.translation.xy())),
        cursor_convert_pos_to_world(&window),
    ) {
        Ok((player, mouse_click)) => {
            // let mut vel = Vec3::default();

            if keyboard_input.just_pressed(KeyCode::Q) {
                let got_mouse_click = cursor_convert_pos_to_world(&window).unwrap();
                let got_vector = (got_mouse_click - player.translation.xy()).normalize();
                let scale_vec = got_vector * 70.;
                // println!("first {got_mouse_click:.2}, after change {scale_vec:#?}");
                let red = xy_to_angle(scale_vec.x, scale_vec.y);
                // println!("ang {:.2}", red);
                let toxy = angle_to_xy(red, 70.);
                // println!(" from ang to xy {:#?}", toxy);
                // vel.y += 1.;
            }

            if keyboard_input.just_pressed(KeyCode::W) {
                // vel.x -= 1.;
            }

            if keyboard_input.just_pressed(KeyCode::E) {
                // vel.x += 1.;
            }

            if keyboard_input.just_pressed(KeyCode::R) {
                // vel.y -= 1.;
            }
        }
        Err((e, None)) => println!("faild {}", e),
    }

    // match input_query.get_single_mut() {
    //     Ok(mut input) => {

    // let mut vel = Vec3::default();

    // if keyboard_input.just_pressed(KeyCode::W) {
    //     let asd = cursor_convert_pos_to_world(&window).unwrap();
    //     let sdfg = asd - player.translation.xy();
    //     println!("first {:#?}, after change {sdfg:#?}", asd);
    //     let red = xy_to_angle(sdfg.x, sdfg.y);
    //     println!("ang {:#?}", red);
    //     let toxy = angle_to_xy(red, 50.);
    //     println!(" from ang to xy{:#?}", toxy);
    //     vel.y += 1.;
    // }

    // if keyboard_input.pressed(KeyCode::A) {
    //     vel.x -= 1.;
    // }

    // if keyboard_input.pressed(KeyCode::D) {
    //     vel.x += 1.;
    // }

    // if keyboard_input.pressed(KeyCode::S) {
    //     vel.y -= 1.;
    // }

    //input.0 = vel;

    //     },
    //     Err(err) => {println!("{err:?}")}
    // };
}

fn cursor_convert_pos_to_world(window: &Window) -> Option<Vec2> {
    if let Some(pos) = window.physical_cursor_position() {
        let size = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );

        // Convert cursor pos to the world
        let world_pos = Vec2::new(pos.x - size.x / 2.0, pos.y - size.y / 2.0);
        return Some(world_pos);
    }

    None
}

/// Convert Cartesian coordinates (x, y) to an angle in radians.
/// The angle is measured in "standard position" -- counter-clockwise from the positive x-axis.
fn xy_to_angle(x: f32, y: f32) -> f32 {
    y.atan2(x)
}

/// Convert an angle in radians to Cartesian coordinates (x, y),
/// given a radius r. Assumes the angle is in "standard position".
fn angle_to_xy(angle: f32, radius: f32) -> (f32, f32) {
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    (x, y)
}
