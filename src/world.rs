use crate::archetype::Archetype;
use crate::entity::Entity;
use std::collections::HashMap;

pub struct World {
    entities: Vec<u32>,
    archetypes: Vec<Archetype>,
    entity_to_archetype: HashMap<u32, (usize, usize)>, // Entity.id -> (arch_index, index_in_archetype)
}

impl World {
    pub fn new() -> Self {
        // Инициализируем пустой мир
        World {
            entities: Vec::new(),
            archetypes: Vec::new(),
            entity_to_archetype: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        // Спавним новую сущность
        let count_of_entities: u32 = self.entities.iter().sum();
        let new_entity_number = count_of_entities + 1;
        let new_entity_id = u32::MAX % new_entity_number;
        match self.entities.get(new_entity_id as usize) {
            Some(value) => {
                let new_entity = Entity {
                    id: new_entity_id,
                    generation: value + 1,
                };
                self.entities
                    .insert(new_entity.id as usize, new_entity.generation);
                new_entity
            }
            None => {
                let new_entity = Entity {
                    id: new_entity_id,
                    generation: 1,
                };
                self.entities
                    .insert(new_entity.id as usize, new_entity.generation);
                new_entity
            }
        }
    }

    pub fn despawn(&mut self, _entity: Entity) -> bool {
        //TODO: реализовать (после работы архетипов)

        // Временная заглушка для работы теста:
        true
    }

    pub fn insert_component<T>(&mut self, _entity: Entity, _component: T) {
        //TODO: реализовать
        unimplemented!()
    }
}
