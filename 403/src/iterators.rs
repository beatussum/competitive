use rayon::iter::{
    ParallelIterator,
    plumbing::{
        Folder, UnindexedConsumer, UnindexedProducer, bridge_unindexed,
    },
};
use scc::HashSet;

type State = (usize, usize);

pub struct StateIterator<'a> {
    root: State,
    has_stone: &'a [bool],
    is_visited: HashSet<State>,
}

impl<'a> StateIterator<'a> {
    pub fn new(root: State, has_stone: &'a [bool]) -> Self {
        Self {
            root,
            has_stone,
            is_visited: HashSet::default(),
        }
    }
}

impl ParallelIterator for StateIterator<'_> {
    type Item = State;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = StateProducer {
            has_stone: &self.has_stone,
            is_visited: &self.is_visited,
            to_visit: vec![self.root],
            to_fold: Vec::default(),
        };

        bridge_unindexed(producer, consumer)
    }
}

pub struct StateProducer<'a> {
    has_stone: &'a [bool],
    is_visited: &'a HashSet<State>,
    to_visit: Vec<State>,
    to_fold: Vec<State>,
}

impl UnindexedProducer for StateProducer<'_> {
    type Item = State;

    fn split(mut self) -> (Self, Option<Self>) {
        let mut to_visit_left = self
            .to_visit
            .iter()
            .copied()
            .filter(|state| self.is_visited.insert(*state).is_ok())
            .flat_map(|(p, s)| {
                let small_speed = s - 1;
                let big_speed = s + 1;
                let big_position = p + big_speed;

                Some((big_position, big_speed))
                    .into_iter()
                    .chain(
                        (small_speed > 0)
                            .then_some((p + small_speed, small_speed)),
                    )
                    .chain(Some((p + s, s)))
                    .filter(|all @ (position, _)| {
                        *position < self.has_stone.len()
                            && !self.is_visited.contains(all)
                            && self.has_stone[*position]
                    })
            })
            .collect::<Vec<_>>();

        let mid = to_visit_left.len() / 2;
        let to_visit_right = to_visit_left.split_off(mid);

        self.to_fold.extend(self.to_visit);

        let mid = self.to_fold.len() / 2;
        let to_fold_right = self.to_fold.split_off(mid);

        let left = Self {
            has_stone: self.has_stone,
            is_visited: self.is_visited,
            to_visit: to_visit_left,
            to_fold: self.to_fold,
        };

        let right = Self {
            has_stone: self.has_stone,
            is_visited: self.is_visited,
            to_visit: to_visit_right,
            to_fold: to_fold_right,
        };

        (
            left,
            (!right.to_visit.is_empty() || !right.to_fold.is_empty())
                .then_some(right),
        )
    }

    fn fold_with<F>(mut self, mut folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        while let Some(all @ (p, s)) = self.to_visit.pop() {
            if self.is_visited.insert(all).is_ok() {
                let small_speed = s - 1;
                let big_speed = s + 1;
                let big_position = p + big_speed;

                let to_visit = Some((big_position, big_speed))
                    .into_iter()
                    .chain(
                        (small_speed > 0)
                            .then_some((p + small_speed, small_speed)),
                    )
                    .chain(Some((p + s, s)))
                    .filter(|all @ (next_position, _)| {
                        *next_position < self.has_stone.len()
                            && !self.is_visited.contains(all)
                            && self.has_stone[*next_position]
                    });

                self.to_visit.extend(to_visit.clone());
                folder = folder.consume_iter(to_visit);
            }
        }

        folder.consume_iter(self.to_fold)
    }
}
