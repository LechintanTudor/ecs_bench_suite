use sparsey::prelude::*;

macro_rules! create_entities {
    ($world:ident; $($variant:ident),*) => {
        $(
            struct $variant(f32);
            $world.register::<$variant>();
            $world.extend((0..20).map(|_| ($variant(0.0), Data(0.0))));
        )*
    };
}

struct Data(f32);

pub struct Benchmark(EntityStorage);

impl Benchmark {
    pub fn new() -> Self {
        let mut entities = EntityStorage::default();
        entities.register::<Data>();
        create_entities!(entities; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(entities)
    }

    pub fn run(&mut self) {
        let mut data = self.0.borrow_mut::<Data>();

        (&mut data).for_each(|data| {
            data.0 *= 2.0;
        });
    }
}
