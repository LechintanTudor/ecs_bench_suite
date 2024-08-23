use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(Clone, Copy, Component)]
struct Transform(Matrix4<f32>);

#[derive(Clone, Copy, Component)]
struct Position(Vector3<f32>);

#[derive(Clone, Copy, Component)]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy, Component)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        world.spawn_batch((0..10_000).map(|_| {
            (
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        self.0
            .query::<(&Velocity, &mut Position)>()
            .iter_mut(&mut self.0)
            .for_each(|(velocity, mut position)| {
                position.0 += velocity.0;
            });
    }
}
