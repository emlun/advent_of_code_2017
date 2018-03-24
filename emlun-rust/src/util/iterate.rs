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
mod tests {
    use iterate;

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
