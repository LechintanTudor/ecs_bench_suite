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
        let mut world = World::builder()
            .register::<Transform>()
            .register::<Position>()
            .register::<Rotation>()
            .register::<Velocity>()
            .build();

        world.extend((0..10_000).map(|_| {
            (
                Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        self.0
            .for_each::<(&mut Position, &Velocity)>(|(position, velocity)| {
                position.0 += velocity.0;
            });
    }
}
