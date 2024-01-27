use cgmath::*;
use sparsey::prelude::*;

#[derive(Clone, Copy)]
struct Transform(Matrix4<f32>);

#[derive(Clone, Copy)]
struct Position(Vector3<f32>);

#[derive(Clone, Copy)]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(EntityStorage);

impl Benchmark {
    pub fn new() -> Self {
        let mut entities = EntityStorage::default();
        entities.register::<Transform>();
        entities.register::<Position>();
        entities.register::<Rotation>();
        entities.register::<Velocity>();

        Self(entities)
    }

    pub fn run(&mut self) {
        self.0.reset();
        self.0.extend((0..10_000).map(|_| {
            (
                Transform(Matrix4::<f32>::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_y()),
                Velocity(Vector3::unit_z()),
            )
        }));
    }
}
