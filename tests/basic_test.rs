use my_ecs::{Entity, World};

#[test]
fn test_spawn_despawn() {
    let mut world = World::new();
    let entity = world.spawn();
    assert!(world.despawn(entity));
}

#[test]
fn test_multy_spawn() {
    let mut world = World::new();
    let entity1 = world.spawn();
    let entity2 = world.spawn();
    assert_ne!(entity1, entity2);
    assert!(world.despawn(entity1));
    assert!(world.despawn(entity2));
}
