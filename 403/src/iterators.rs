use rayon::iter::{
    ParallelIterator,
    plumbing::{
        Folder, UnindexedConsumer, UnindexedProducer, bridge_unindexed,
    },
};

use super::State;
use scc::HashSet;

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
        use ToVisit::*;

        let producer = StateProducer {
            has_stone: &self.has_stone,
            is_visited: &self.is_visited,
            to_visit: One(self.root),
            ancestors: Vec::default(),
        };

        bridge_unindexed(producer, consumer)
    }
}

#[derive(Debug)]
enum ToVisit {
    Zero,
    One(State),
    Two(State, State),
}

impl Into<Vec<State>> for ToVisit {
    fn into(self) -> Vec<State> {
        use ToVisit::*;

        match self {
            Zero => Vec::default(),
            One(a) => vec![a],
            Two(a, b) => vec![a, b],
        }
    }
}

pub struct StateProducer<'a> {
    has_stone: &'a [bool],
    is_visited: &'a HashSet<State>,
    to_visit: ToVisit,
    ancestors: Vec<State>,
}

impl UnindexedProducer for StateProducer<'_> {
    type Item = State;

    fn split(self) -> (Self, Option<Self>) {
        use ToVisit::*;

        match self.to_visit {
            Zero => (self, None),

            One(root @ (p, s)) => {
                if self.is_visited.insert(root).is_err() {
                    let left = Self {
                        has_stone: self.has_stone,
                        is_visited: self.is_visited,
                        to_visit: Zero,
                        ancestors: self.ancestors,
                    };

                    (left, None)
                } else {
                    let mut ancestors = self.ancestors;
                    ancestors.push(root);

                    let small_speed = s - 1;
                    let big_speed = s + 1;
                    let big_position = p + big_speed;

                    let mut to_visit = Some((big_position, big_speed))
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
                        });

                    let to_visit =
                        (to_visit.next(), to_visit.next(), to_visit.next());

                    match to_visit {
                        (None, None, None) => {
                            let left = Self {
                                has_stone: self.has_stone,
                                is_visited: self.is_visited,
                                to_visit: Zero,
                                ancestors,
                            };

                            (left, None)
                        }

                        (Some(a), None, None) => {
                            let left = Self {
                                has_stone: self.has_stone,
                                is_visited: self.is_visited,
                                to_visit: One(a),
                                ancestors: Vec::default(),
                            };

                            let right = Self {
                                has_stone: self.has_stone,
                                is_visited: self.is_visited,
                                to_visit: Zero,
                                ancestors,
                            };

                            (left, Some(right))
                        }

                        (Some(a), Some(b), None) => {
                            let mid = ancestors.len() / 2;
                            let right_ancestors = ancestors.split_off(mid);

                            let left = Self {
                                has_stone: self.has_stone,
                                is_visited: self.is_visited,
                                to_visit: One(a),
                                ancestors,
                            };

                            let right = Self {
                                has_stone: self.has_stone,
                                is_visited: self.is_visited,
                                to_visit: One(b),
                                ancestors: right_ancestors,
                            };

                            (left, Some(right))
                        }

                        (Some(a), Some(b), Some(c)) => {
                            let left = Self {
                                has_stone: self.has_stone,
                                is_visited: self.is_visited,
                                to_visit: One(a),
                                ancestors: Vec::default(),
                            };

                            let right = Self {
                                has_stone: self.has_stone,
                                is_visited: self.is_visited,
                                to_visit: Two(b, c),
                                ancestors,
                            };

                            (left, Some(right))
                        }

                        _ => unreachable!(),
                    }
                }
            }

            Two(a, b) => {
                let mut left_ancestors = self.ancestors;
                let mid = left_ancestors.len();
                let right_ancestors = left_ancestors.split_off(mid);

                let left = Self {
                    has_stone: self.has_stone,
                    is_visited: self.is_visited,
                    to_visit: One(a),
                    ancestors: left_ancestors,
                };

                let right = Self {
                    has_stone: self.has_stone,
                    is_visited: self.is_visited,
                    to_visit: One(b),
                    ancestors: right_ancestors,
                };

                (left, Some(right))
            }
        }
    }

    fn fold_with<F>(self, mut folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        let mut to_visit: Vec<State> = self.to_visit.into();

        if folder.full() {
            folder
        } else {
            while let Some(all @ (p, s)) = to_visit.pop() {
                if self.is_visited.insert(all).is_ok() {
                    let small_speed = s - 1;
                    let big_speed = s + 1;
                    let big_position = p + big_speed;

                    let extra = Some((big_position, big_speed))
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
                        });

                    folder = folder.consume_iter(extra.clone());

                    if folder.full() {
                        return folder;
                    } else {
                        to_visit.extend(extra);
                    }
                }
            }

            folder.consume_iter(self.ancestors)
        }
    }
}
