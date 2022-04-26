use crate::components::Boid;
use bevy::prelude::Component;
use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct Flock {
    boids: Vec<Boid>,
}

impl Flock {
    pub fn new(n: usize) -> Self {
        let mut boids = Vec::new();
        for _ in 0..n {
            boids.push(Boid::new(0., 0.));
        }
        Self { boids }
    }

    pub fn boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    pub fn len(&self) -> usize {
        self.boids.len()
    }

    pub fn run(&self) {
        for boid in self.boids.clone().iter_mut() {
            boid.run(self.boids.clone());
        }
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
