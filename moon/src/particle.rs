//! THe [`Particle`] and [`ParticleSystem`].

use nalgebra::{Vector2, Vector4};

use crate::transform::Transform2D;

const MAX_PARTICLES: usize = 100;

pub enum ParticleValue<T: Copy> {
    Constant(T),
    Linear(T, T),
    Random(T, T),
}

impl<T: Default + Copy> Default for ParticleValue<T> {
    fn default() -> Self {
        ParticleValue::Constant(T::default())
    }
}

impl<T: Copy> ParticleValue<T> {
    pub fn get_value(&self, _time: f32) -> T {
        match self {
            ParticleValue::Constant(value) => *value,
            ParticleValue::Linear(_min, _max) => todo!(),
            ParticleValue::Random(_min, _max) => *_min,
        }
    }
}

pub struct ParticleProps {
    pub lifetime: ParticleValue<f32>,
    pub velocity: ParticleValue<Vector2<f32>>,
    pub color: ParticleValue<Vector4<f32>>,
    pub size: ParticleValue<f32>,
}

impl Default for ParticleProps {
    fn default() -> Self {
        Self {
            lifetime: ParticleValue::Constant(10.0),
            velocity: ParticleValue::Constant(Vector2::new(0.0, -1.0)),
            color: ParticleValue::Constant(Vector4::new(1.0, 1.0, 1.0, 1.0)),
            size: ParticleValue::Constant(1.0),
        }
    }
}

pub struct Particle {
    transform: Transform2D,
    properties: ParticleProps,
    lifetime: f32,
    velocity: Vector2<f32>,
    age: f32,
    alive: bool,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            properties: Default::default(),
            lifetime: 10.0,
            velocity: Vector2::new(0.0, -1.0),
            age: 0.0,
            alive: true,
        }
    }
}

impl Particle {
    pub fn update(&mut self, delta_time: f32) {
        self.age += delta_time;
        if self.age > self.lifetime {
            self.alive = false;
        } else {
            self.transform.position += self.velocity;
        }
    }
}

struct ParticleSystem {
    particles: Vec<Particle>,
}
