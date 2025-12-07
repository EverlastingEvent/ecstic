use criterion::{Criterion, black_box, criterion_group, criterion_main};
use my_ecs::World;

fn bench_spawn_entities(c: &mut Criterion) {
    c.bench_function("spawn_entities", |b| {
        b.iter(|| {
            let mut world = World::new();
            for _ in 0..1000 {
                black_box(world.spawn());
            }
        });
    });
}

criterion_group!(benches, bench_spawn_entities);
criterion_main!(benches);
