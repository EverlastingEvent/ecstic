use my_ecs::{World, Entity};

#[test]
fn test_spawn_despawn() {
    let mut world = World::new();
    let entity = world.spawn();
    assert!(world.despawn(entity));
}