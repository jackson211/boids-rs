mod boid;
mod flock;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use flock::*;
use systems::*;

const N_BOIDS: usize = 10;

fn main() {
    App::new()
        // .insert_resource(Msaa { samples: 4 })
        .insert_resource(GameState {
            flock: Flock::new(N_BOIDS, 100., 100.),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(boid_movement)
        .run();
}

fn setup(mut commands: Commands, game_state: ResMut<GameState>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    run(commands, game_state);
}
