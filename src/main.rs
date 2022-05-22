mod boid;
mod flock;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use flock::*;
use systems::*;

const N_BOIDS: usize = 10;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

fn main() {
    App::new()
        // .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "boids".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .insert_resource(GameState {
            flock: Flock::new(N_BOIDS, 100., 100.),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(BoidPlugin)
        .add_startup_system(setup)
        .add_system(boid_movement)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
}
