use sparsey::prelude::*;

struct A(f32);
struct B(f32);

pub struct Benchmark(EntityStorage, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut entities = EntityStorage::default();
        entities.register::<A>();
        entities.register::<B>();

        let entity_vec = entities.extend((0..10_000).map(|_| (A(0.0),))).to_vec();

        Self(entities, entity_vec)
    }

    pub fn run(&mut self) {
        for &entity in &self.1 {
            self.0.insert(entity, (A(0.0), B(0.0)));
        }

        for &entity in &self.1 {
            self.0.delete::<(A, B)>(entity);
        }
    }
}
