use crate::components::*;
use crate::resources::*;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct GameState {
    pub flock: Flock,
}

pub fn run(mut commands: Commands, game_state: ResMut<GameState>) {
    let boids = game_state.flock.boids();

    let triangle = shapes::Polygon {
        points: vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 10.0),
            Vec2::new(25., 5.),
        ],
        closed: true,
    };

    // let mut cmd = commands.spawn();
    for i in 0..boids.len() {
        let boid = boids[i];
        let pos = boid.position();
        let mut transform = Transform::from_translation(Vec3::new(pos.x, pos.y, 0.));
        transform.rotate(Quat::from_rotation_z(-boid.angle()));
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &triangle,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 2.),
                },
                transform,
            ))
            .insert(boid);
    }
}

// Boids movement
pub fn boid_movement(
    mut boids_position: Query<(&mut Boid, &mut Transform)>,
    game_state: ResMut<GameState>,
) {
    let boids = game_state.flock.boids();
    for (mut boid, mut transform) in boids_position.iter_mut() {
        boid.run(boids.clone());
        let pos = boid.position();
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
        // transform.rotate(Quat::from_rotation_z(-boid.angle()));
    }
}
