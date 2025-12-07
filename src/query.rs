use crate::world::World;

pub trait Query {
    type Item<'a>;
    fn iter<'a>(world: &'a World) -> Box<dyn Iterator<Item = Self::Item<'a>> + 'a>;
}
