use crate::world::World;

pub struct WorldQueryIter<'a, T> {
    data: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for WorldQueryIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next()
    }
}

pub struct Query<'a, T> {
    world: &'a World,
    _maker: std::marker::PhantomData<T>,
}

impl<'a, T: 'static> Query<'a, T> {
    pub fn new(world: &'a World) -> Self {
        Self {
            world,
            _maker: std::marker::PhantomData,
        }
    }

    pub fn iter(self) -> WorldQueryIter<'a, T> {
        // Здесь будет сложная логика сбора данных из Архетипов
        // Пока для примера представм, что доастали один вектор:
        // Тут появится unsafe или RefCell
        // чтобы достать данные из World
        unimplemented!("Нужно реализовать выборку из архетипов")
    }
}
