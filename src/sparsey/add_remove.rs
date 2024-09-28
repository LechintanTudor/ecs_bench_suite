use sparsey::{Entity, World};

struct A(f32);
struct B(f32);

pub struct Benchmark {
    world: World,
    entities: Vec<Entity>,
}

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::builder().register::<A>().register::<B>().build();
        let entities = world.extend((0..10_000).map(|_| (A(0.0),))).to_vec();
        Self { world, entities }
    }

    pub fn run(&mut self) {
        for &entity in &self.entities {
            self.world.insert(entity, (A(0.0), B(0.0)));
        }

        for &entity in &self.entities {
            self.world.delete::<(A, B)>(entity);
        }
    }
}
