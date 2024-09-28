use cgmath::*;
use sparsey::World;

#[derive(Clone, Copy)]
struct Transform(Matrix4<f32>);

#[derive(Clone, Copy)]
struct Position(Vector3<f32>);

#[derive(Clone, Copy)]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World::builder()
            .register::<Transform>()
            .register::<Position>()
            .register::<Rotation>()
            .register::<Velocity>()
            .register::<Velocity>()
            .build();

        Self(world)
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
