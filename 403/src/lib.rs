use iterators::StateIterator;
use rayon::prelude::*;

mod iterators;
pub mod solve;

pub use solve::solve;

pub type State = (usize, usize);

#[derive(Clone)]
pub struct Input {
    pub has_stone: Vec<bool>,
    pub root: State,
}

impl Input {
    pub fn len(&self) -> usize {
        self.has_stone.len()
    }
}

impl IntoParallelIterator for Input {
    type Iter = StateIterator<Vec<bool>>;
    type Item = State;

    fn into_par_iter(self) -> Self::Iter {
        StateIterator::new(self.root, self.has_stone)
    }
}

impl<'a> IntoParallelRefIterator<'a> for Input {
    type Iter = StateIterator<&'a [bool]>;
    type Item = State;

    fn par_iter(&'a self) -> Self::Iter {
        StateIterator::new(self.root, self.has_stone.as_slice())
    }
}

pub fn parse_input<I>(input: I) -> Vec<bool>
where
    I: IntoIterator<Item = usize>,
{
    use itertools::Itertools;
    use std::iter::once;

    let input = input
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a - 1)
        .flat_map(|prefix| once(false).cycle().take(prefix).chain(once(true)));

    once(true).chain(input).collect()
}
