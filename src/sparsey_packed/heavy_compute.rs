use cgmath::*;
use sparsey::World;

#[derive(Clone, Copy)]
struct Matrix(Matrix4<f32>);

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
            .add_group::<(Matrix, Position)>()
            .register::<Rotation>()
            .register::<Velocity>()
            .build();

        world.extend((0..1000).map(|_| {
            (
                Matrix(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        self.0
            .par_for_each::<(&mut Position, &mut Matrix)>(|(pos, mat)| {
                for _ in 0..100 {
                    mat.0 = mat.0.invert().unwrap();
                }

                pos.0 = mat.0.transform_vector(pos.0);
            });
    }
}
