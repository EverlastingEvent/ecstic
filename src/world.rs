use crate::archetype::Archetype;
use crate::entity::Entity;
use std::collections::HashMap;

pub struct World {
    entities: Vec<u32>,
    free_ids: Vec<u32>,
    archetypes: Vec<Archetype>,
    entity_to_archetype: HashMap<u32, (usize, usize)>, // Entity.id -> (arch_index, index_in_archetype)
}

impl World {
    pub fn new() -> Self {
        // Инициализируем пустой мир
        World {
            entities: Vec::new(),
            free_ids: Vec::new(),
            archetypes: Vec::new(),
            entity_to_archetype: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        // Спавним новую сущность
        if let Some(reused_id) = self.free_ids.pop() {
            let generation = &mut self.entities[reused_id as usize];
            // такая запись горантирует, что старые ссылки на сущность будут удалены
            *generation += 1;
            Entity {
                id: reused_id,
                generation: *generation,
            }
        } else {
            let new_id = self.entities.len() as u32;
            self.entities.push(1);
            Entity {
                id: new_id,
                generation: 1,
            }
        }
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        // Проверка: существует ли ID и совпадает ли поколение
        if let Some(&genir) = self.entities.get(entity.id as usize) {
            if genir == entity.generation {
                self.free_ids.push(entity.id);
                //TODO: реализовать (после работы архетипов)
                return true;
            }
        }
        false
    }

    pub fn insert_component<T>(&mut self, _entity: Entity, _component: T) {
        //TODO: реализовать
        unimplemented!()
    }
}
