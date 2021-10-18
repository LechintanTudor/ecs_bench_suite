use cgmath::*;
use sparsey::prelude::*;

struct Transform(Matrix4<f32>);
struct Position(Vector3<f32>);
struct Rotation(Vector3<f32>);
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.register::<Transform>();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Velocity>();

        world.create_entities((0..10_000).map(|_| {
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
        let (mut positions, velocities) = self.0.borrow::<(CompMut<Position>, Comp<Velocity>)>();

        (&mut positions, &velocities)
            .iter()
            .for_each(|(mut position, velocity)| {
                position.0 += velocity.0;
            })
    }
}
