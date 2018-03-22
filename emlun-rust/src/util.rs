pub struct Flatten<Outer, Inner, Item>
    where Outer: IntoIterator<Item=Inner>,
          Inner: IntoIterator<Item=Item>,
{
    current_iterator: Option<Inner::IntoIter>,
    iterators: Outer::IntoIter,
}
impl<Outer, Inner, Item> Iterator for Flatten<Outer, Inner, Item>
    where Outer: IntoIterator<Item=Inner>,
          Inner: IntoIterator<Item=Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        if let Some(mut current_iterator) = self.current_iterator.take() {
            match current_iterator.next() {
                Some(next) => {
                    self.current_iterator = Some(current_iterator);
                    Some(next)
                },
                None => {
                    self.current_iterator = None;
                    self.next()
                }
            }
        } else {
            match self.iterators.next() {
                Some(next_iterator) => {
                    self.current_iterator = Some(next_iterator.into_iter());
                    self.next()
                },
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod flatten_tests {
    use util::Flattenable;

    #[test]
    fn empty_vectors() {
        assert_eq!(vec![vec![], vec![]].flatten().collect::<Vec<i32>>(), vec![]);
    }

    #[test]
    fn empty_and_singleton_vector() {
        assert_eq!(vec![vec![], vec![0]].flatten().collect::<Vec<i32>>(), vec![0]);
    }

    #[test]
    fn singleton_and_empty_vector() {
        assert_eq!(vec![vec![0], vec![]].flatten().collect::<Vec<i32>>(), vec![0]);
    }

    #[test]
    fn singleton_and_singleton_vector() {
        assert_eq!(vec![vec![0], vec![1]].flatten().collect::<Vec<i32>>(), vec![0, 1]);
    }

    #[test]
    fn nontrivial_vectors() {
        assert_eq!(
            vec![
                vec![0, 1, 2],
                vec![3, 4, 5],
                vec![6, 7, 8]
            ]
                .flatten()
                .collect::<Vec<i32>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
    }
}

pub trait Flattenable<Outer, Inner, Item>
    where Outer: IntoIterator<Item=Inner>,
          Inner: IntoIterator<Item=Item>,
{
    fn flatten(self) -> Flatten<Outer, Inner, Item>;
}

impl<Outer, Inner, Item> Flattenable<Outer, Inner, Item> for Outer
    where Outer: IntoIterator<Item=Inner>,
          Inner: IntoIterator<Item=Item>,
{
    fn flatten(self) -> Flatten<Outer, Inner, Item> {
        Flatten {
            current_iterator: None,
            iterators: self.into_iter(),
        }
    }
}

pub struct ApplyIterator<T, F>
    where F: Fn(&T) -> T
{
    state: Option<T>,
    func: F,
}

impl<T, F> ApplyIterator<T, F>
    where F: Fn(&T) -> T
{
    fn new(state: T, func: F) -> ApplyIterator<T, F> {
        ApplyIterator {
            state: Some(state),
            func: func,
        }
    }

}

impl<T, F> Iterator for ApplyIterator<T, F>
    where F: Fn(&T) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current_state = self.state.take().unwrap();
        self.state = Some((self.func)(&current_state));
        Some(current_state)
    }
}

pub fn iterate<T, F>(seed: T, f: F) -> ApplyIterator<T, F>
    where F: Fn(&T) -> T
{
    ApplyIterator::new(seed, f)
}


#[cfg(test)]
mod iterate_tests {
    use util::iterate;

    #[test]
    fn stationary() {
        assert_eq!(iterate(7, |&a| a).take(5).collect::<Vec<i32>>(), vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn increment() {
        assert_eq!(iterate(0, |&a| a + 1).take(5).collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn alternating() {
        assert_eq!(
            iterate(false, |&a| !a).take(4).collect::<Vec<bool>>(),
            vec![false, true, false, true]
        );
    }

    #[test]
    fn endless() {
        assert_eq!(
            iterate(false, |&a| !a).take(10000).collect::<Vec<bool>>(),
            vec![false, true].iter().cycle().map(|&b| b).take(10000).collect::<Vec<bool>>()
        );
    }
}
