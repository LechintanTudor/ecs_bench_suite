use cgmath::Transform as _;
use cgmath::*;
use rayon::prelude::*;
use shipyard::*;

#[derive(Component, Clone, Copy)]
struct Position(Vector3<f32>);

#[derive(Component, Clone, Copy)]
struct Rotation(Vector3<f32>);

#[derive(Component, Clone, Copy)]
struct Velocity(Vector3<f32>);

#[derive(Component, Clone, Copy)]
struct Transform(Matrix4<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let world = World::default();

        world.run(
            |mut entities: EntitiesViewMut,
             mut transforms: ViewMut<Transform>,
             mut positions: ViewMut<Position>,
             mut rotations: ViewMut<Rotation>,
             mut velocities: ViewMut<Velocity>| {
                for _ in 0..1000 {
                    entities.add_entity(
                        (
                            &mut transforms,
                            &mut positions,
                            &mut rotations,
                            &mut velocities,
                        ),
                        (
                            Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                            Position(Vector3::unit_x()),
                            Rotation(Vector3::unit_x()),
                            Velocity(Vector3::unit_x()),
                        ),
                    );
                }
            },
        );

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.run(
            |mut positions: ViewMut<Position>, mut transforms: ViewMut<Transform>| {
                (&mut positions, &mut transforms)
                    .par_iter()
                    .for_each(|(pos, mat)| {
                        for _ in 0..100 {
                            mat.0 = mat.0.invert().unwrap();
                        }
                        pos.0 = mat.0.transform_vector(pos.0);
                    });
            },
        );
    }
}
