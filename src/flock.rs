use crate::boid::Boid;
use bevy::prelude::Component;
use core::ops::{Deref, DerefMut};
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Component)]
pub struct Flock {
    boids: Vec<Boid>,
}

impl Flock {
    pub fn new(n: usize, width: f32, height: f32) -> Self {
        let mut boids = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            boids.push(Boid::new(
                rng.gen_range(0f32..width),
                rng.gen_range(0f32..height),
            ));
        }
        Self { boids }
    }

    pub fn boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    pub fn len(&self) -> usize {
        self.boids.len()
    }
}

impl Deref for Flock {
    type Target = Vec<Boid>;
    fn deref(&self) -> &Self::Target {
        &self.boids
    }
}

impl DerefMut for Flock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.boids
    }
}
