use sparsey::World;

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

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::builder().register::<Data>().build();
        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
        Self(world)
    }

    pub fn run(&mut self) {
        self.0.for_each::<&mut Data>(|data| {
            data.0 *= 2.0;
        });
    }
}
