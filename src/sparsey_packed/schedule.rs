use rayon::{ThreadPool, ThreadPoolBuilder};
use sparsey::prelude::*;

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab(mut a: CompMut<A>, mut b: CompMut<B>) {
    (&mut a, &mut b).for_each(|(a, b)| {
        std::mem::swap(&mut a.0, &mut b.0);
    });
}

fn cd(mut c: CompMut<C>, mut d: CompMut<D>) {
    (&mut c, &mut d).for_each(|(c, d)| {
        std::mem::swap(&mut c.0, &mut d.0);
    });
}

fn ce(mut c: CompMut<C>, mut e: CompMut<E>) {
    (&mut c, &mut e).for_each(|(c, e)| {
        std::mem::swap(&mut c.0, &mut e.0);
    });
}

pub struct Benchmark(EntityStorage, ThreadPool);

impl Benchmark {
    pub fn new() -> Self {
        let layout = GroupLayout::builder()
            .add_group::<(A, B)>()
            .add_group::<(C, D)>()
            .build();

        let mut entities = EntityStorage::new(&layout);
        entities.register::<E>();

        entities.extend((0..10_000).map(|_| (A(0.0),)));
        entities.extend((0..10_000).map(|_| (A(0.0), B(0.0))));
        entities.extend((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0))));
        entities.extend((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));
        entities.extend((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let thread_pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();

        Self(entities, thread_pool)
    }

    pub fn run(&mut self) {
        self.1.scope(|scope| {
            scope.spawn(|_| {
                self.0.run(ab);
            });

            scope.spawn(|_| {
                self.0.run(cd);
            });
        });

        self.0.run(ce);
    }
}
