use rayon::{ThreadPool, ThreadPoolBuilder};
use sparsey::World;

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

pub struct Benchmark {
    world: World,
    thread_pool: ThreadPool,
}

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::builder()
            .add_group::<(A, B)>()
            .add_group::<(C, D)>()
            .register::<E>()
            .build();

        world.extend((0..10_000).map(|_| (A(0.0),)));
        world.extend((0..10_000).map(|_| (A(0.0), B(0.0))));
        world.extend((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0))));
        world.extend((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));
        world.extend((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        Benchmark {
            world,
            thread_pool: ThreadPoolBuilder::new().num_threads(2).build().unwrap(),
        }
    }

    pub fn run(&mut self) {
        self.thread_pool.scope(|scope| {
            scope.spawn(|_| {
                self.world.for_each::<(&mut A, &mut B)>(|(a, b)| {
                    std::mem::swap(&mut a.0, &mut b.0);
                });
            });

            scope.spawn(|_| {
                self.world.for_each::<(&mut C, &mut D)>(|(c, d)| {
                    std::mem::swap(&mut c.0, &mut d.0);
                });
            });
        });

        self.world.for_each::<(&mut C, &mut E)>(|(c, e)| {
            std::mem::swap(&mut c.0, &mut e.0);
        });
    }
}
