use std::collections::HashMap;
use std::hash::Hash;

pub trait Countable<'a, T>
    where T: Hash + Eq,
          T: 'a,
{
    fn counts(self) -> HashMap<&'a T, u32>;
}

impl<'a, I, T> Countable<'a, T> for I
    where I: Iterator<Item=&'a T>,
          T: Hash + Eq,
          T: 'a,
{
    fn counts(self) -> HashMap<&'a T, u32> {
        self.fold(
            HashMap::new(),
            |mut map, item| {
                let current_count = *map.entry(&item).or_insert(0);
                map.insert(&item, current_count + 1);
                map
            }
        )
    }
}
