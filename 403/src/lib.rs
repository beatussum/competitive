use iterators::StateIterator;
use rayon::prelude::*;

mod iterators;
pub mod solve;

pub use solve::solve;

pub type State = (usize, usize);

#[derive(Clone, Copy)]
pub struct Input<'a> {
    has_stone: &'a [bool],
    root: State,
}

impl<'a> Input<'a> {
    pub fn new(has_stone: &'a [bool], root: State) -> Self {
        Self { has_stone, root }
    }

    pub fn len(&self) -> usize {
        self.has_stone.len()
    }
}

impl<'a> IntoParallelIterator for Input<'a> {
    type Iter = StateIterator<'a>;
    type Item = State;

    fn into_par_iter(self) -> Self::Iter {
        StateIterator::new(self.root, self.has_stone)
    }
}

impl<'a> IntoParallelRefIterator<'a> for Input<'a> {
    type Iter = StateIterator<'a>;
    type Item = State;

    fn par_iter(&'a self) -> Self::Iter {
        StateIterator::new(self.root, self.has_stone)
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
