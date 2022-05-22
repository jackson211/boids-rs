use bevy::prelude::{Component, Vec2};

/// Boid component
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    r: f32,
    maxforce: f32,
    maxspeed: f32,
    angle: f32,
}

impl Boid {
    pub fn new(x: f32, y: f32) -> Boid {
        let angle: f32 = 1. * std::f32::consts::PI * rand::random::<f32>();
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(f32::cos(angle), f32::sin(angle)),
            acceleration: Vec2::new(0., 0.),
            r: 2.,
            maxforce: 0.03,
            maxspeed: 2.,
            angle,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(self.maxspeed);
        self.position += self.velocity;
        self.acceleration *= Vec2::new(0., 0.);
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn run(&mut self, boids: Vec<Boid>) {
        self.flock(boids);
        self.update();
    }

    /// Flock
    pub fn flock(&mut self, boids: Vec<Boid>) {
        let sep = self.separate(boids.clone()) * 1.5; // Apply weights
        let ali = self.align(boids.clone()) * 1.0;
        let coh = self.cohesion(boids) * 1.0;
        self.apply_force(sep);
        self.apply_force(ali);
        self.apply_force(coh);
    }

    /// Separation
    /// checks for nearby boids and steer
    pub fn separate(&self, boids: Vec<Boid>) -> Vec2 {
        let desiredseparation = 25.0f32;
        let mut steer = Vec2::new(0., 0.);
        let position = self.position;

        let mut count = 0;
        for other in boids.iter() {
            let other_position = other.position();
            let d = position.distance(other_position);
            if (d > 0.) && (d < desiredseparation) {
                // Calculate vector pointing away from neighbor
                let mut diff = position - other_position;
                diff = diff.normalize();
                diff /= d; // Weight by distance
                steer += diff;
                count += 1; // Keep track of how many
            }
        }

        if count > 0 {
            steer /= Vec2::new(count as f32, count as f32);
        }

        if steer.length() > 0.0 {
            steer = steer.normalize();
            steer *= self.maxspeed;
            steer -= self.velocity;
            steer = steer.clamp_length_max(self.maxforce);
        }
        return steer;
    }

    /// Alignment
    /// Calculate the average velocity of nearby boids in the system
    pub fn align(&self, boids: Vec<Boid>) -> Vec2 {
        let neighbordist: f32 = 50.;
        let mut sum = Vec2::new(0., 0.);
        let mut count = 0;
        let position = self.position;

        for other in boids.iter() {
            let d = position.distance(other.position());
            if (d > 0.) && (d < neighbordist) {
                sum += other.velocity();
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f32;
            sum = sum.normalize();
            sum *= self.maxspeed;
            let mut steer = sum - self.velocity;
            steer = steer.clamp_length_max(self.maxforce);
            return steer;
        }

        Vec2::new(0., 0.)
    }

    /// Cohesion
    /// Find the average center of mass for all nearby boids, calculate steering vector
    /// towards that position
    pub fn cohesion(&self, boids: Vec<Boid>) -> Vec2 {
        let neighbordist = 50.;
        let mut sum = Vec2::new(0., 0.);
        let mut count = 0;
        let position = self.position;

        for other in boids.iter() {
            let d = position.distance(other.position());
            if (d > 0.) && (d < neighbordist) {
                sum += other.velocity();
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f32;
            return self.seek(sum); //Steer towards the position
        }

        Vec2::new(0., 0.)
    }

    /// Calculate the steering force towards a target
    fn seek(&self, target: Vec2) -> Vec2 {
        let mut desired = target - self.position();
        desired = desired.normalize();
        desired *= self.maxspeed;

        let mut steer = desired - self.velocity;
        steer = steer.clamp_length_max(self.maxforce);
        return steer;
    }
}
