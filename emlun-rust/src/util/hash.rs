use std::hash::Hash;
use std::hash::Hasher;

pub trait Hashable
    where Self: Hash
{
    fn finish_hash(&self) -> u64 {
        // let hasher = std::hash::Hasher;
        let mut hasher = ::std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T> Hashable for T
    where T: Hash
{
}
