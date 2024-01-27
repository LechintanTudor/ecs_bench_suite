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
        let layout = GroupLayout::builder()
            .add_group::<(Position, Velocity)>()
            .build();

        let mut entities = EntityStorage::new(&layout);
        entities.register::<Transform>();
        entities.register::<Rotation>();

        entities.extend((0..10_000).map(|_| {
            (
                Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(entities)
    }

    pub fn run(&mut self) {
        self.0.run(
            |mut positions: CompMut<Position>, velocities: Comp<Velocity>| {
                (&mut positions, &velocities).for_each(|(position, velocity)| {
                    position.0 += velocity.0;
                });
            },
        );
    }
}
