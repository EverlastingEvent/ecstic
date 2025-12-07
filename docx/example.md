```rust
use my_ecs::{World, Entity, Query, System, Schedule};

// === 1. Определяем компоненты ===

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Debug)]
struct Health {
    value: u32,
}

#[derive(Debug)]
struct PlayerInput;

// === 2. Создаём мир ===

let mut world = World::new();

// === 3. Создаём 10 NPC и 1 игрока ===

// NPC: Position + Velocity + Health
for _ in 0..10 {
    let npc = world.spawn();
    world.insert_component(npc, Position { x: 0.0, y: 0.0 });
    world.insert_component(npc, Velocity { dx: 0.1, dy: 0.0 });
    world.insert_component(npc, Health { value: 100 });
}

// Игрок: Position + Health + PlayerInput
let player = world.spawn();
world.insert_component(player, Position { x: 5.0, y: 5.0 });
world.insert_component(player, Health { value: 100 });
world.insert_component(player, PlayerInput);

// === 4. Определяем Query для NPC и игрока ===

// Это будет реализовано позже, но выглядеть будет примерно так:
// struct QueryNPC;
// impl Query for QueryNPC {
//     type Item<'a> = (&'a mut Position, &'a Velocity, &'a Health);
//     fn iter<'a>(world: &'a World) -> Box<dyn Iterator<Item = Self::Item<'a>> + 'a> {
//         // возвращает итератор по Position, Velocity, Health
//     }
// }

// struct QueryPlayer;
// impl Query for QueryPlayer {
//     type Item<'a> = (&'a mut Position, &'a Health, &'a PlayerInput);
//     fn iter<'a>(world: &'a World) -> Box<dyn Iterator<Item = Self::Item<'a>> + 'a> {
//         // возвращает итератор по Position, Health, PlayerInput
//     }
// }

// === 5. Создаём системы ===

struct NpcMovementSystem;

impl System for NpcMovementSystem {
    fn run(&mut self, world: &mut World) {
        // Используем QueryNPC (псевдокод)
        // for (mut pos, vel, _health) in QueryNPC::iter(world) {
        //     pos.x += vel.dx;
        //     pos.y += vel.dy;
        // }
        println!("NPC движутся...");
    }
}

struct PlayerInputSystem;

impl System for PlayerInputSystem {
    fn run(&mut self, world: &mut World) {
        // Используем QueryPlayer (псевдокод)
        // for (mut pos, _health, _input) in QueryPlayer::iter(world) {
        //     // обработка ввода, например, pos.x += 1.0
        // }
        println!("Обработка ввода игрока...");
    }
}

// === 6. Создаём планировщик и добавляем туда системы ===

let mut schedule = Schedule::new();
schedule.add_system(Box::new(NpcMovementSystem));
schedule.add_system(Box::new(PlayerInputSystem));

// === 7. Запускаем все системы ===

schedule.run(&mut world);
```