use crate::{Input, State};

pub fn dfs_solve(input: Input) -> bool {
    use std::collections::HashSet;

    let len = input.len();

    let mut is_visited = HashSet::new();
    let mut to_visit = vec![input.root];

    while let Some(state @ (p, s)) = to_visit.pop() {
        if p == len - 1 {
            return true;
        } else if is_visited.insert(state) {
            let small_speed = s - 1;
            let big_speed = s + 1;
            let big_position = p + big_speed;

            let next = Some((big_position, big_speed))
                .into_iter()
                .chain(
                    (small_speed > 0).then_some((p + small_speed, small_speed)),
                )
                .chain(Some((p + s, s)))
                .filter(|all @ &(p, _)| {
                    (p < len) && input.has_stone[p] && !is_visited.contains(all)
                });

            to_visit.extend(next)
        }
    }

    false
}

#[cfg(feature = "dfs")]
pub fn solve(input: Input) -> bool {
    dfs_solve(input)
}

pub fn iterative_solve(input: Input) -> bool {
    let len = input.len();

    let mut is_solvable_with = (0..len)
        .map(|p| (0..len).map(|_| p == len - 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    input
        .has_stone
        .iter()
        .copied()
        .enumerate()
        .rev()
        .filter_map(|(i, has_stone)| Some(i).filter(|_| has_stone))
        .for_each(|p| {
            if let Some((first_position, other_positions)) =
                is_solvable_with[p..].split_first_mut()
            {
                first_position[..len - p]
                    .iter_mut()
                    .enumerate()
                    .skip(1)
                    .for_each(|(s, is_solvable)| {
                        *is_solvable = other_positions[s - 1][s]
                            || other_positions[s][s + 1];
                    });
            }
        });

    is_solvable_with[0][1]
}

#[cfg(feature = "iterative")]
pub fn solve(input: Input) -> bool {
    iterative_solve(input)
}

pub fn par_solve(input: Input) -> bool {
    use rayon::prelude::*;

    let len = input.len();

    let mut is_solvable_with = (0..len)
        .map(|p| (0..len).map(|_| p == len - 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    input
        .has_stone
        .iter()
        .copied()
        .enumerate()
        .rev()
        .filter_map(|(i, has_stone)| Some(i).filter(|_| has_stone))
        .for_each(|p| {
            if let Some((first_position, other_positions)) =
                is_solvable_with[p..].split_first_mut()
            {
                let speeds = &mut first_position[..len - p];

                if speeds.len() > 500 {
                    speeds
                        .par_iter_mut()
                        .with_min_len(200)
                        .enumerate()
                        .skip(1)
                        .for_each(|(s, is_solvable)| {
                            *is_solvable = other_positions[s - 1][s]
                                || other_positions[s][s + 1];
                        });
                } else {
                    speeds.iter_mut().enumerate().skip(1).for_each(
                        |(s, is_solvable)| {
                            *is_solvable = other_positions[s - 1][s]
                                || other_positions[s][s + 1];
                        },
                    );
                };
            }
        });

    is_solvable_with[0][1]
}

#[cfg(feature = "par")]
pub fn solve(input: Input) -> bool {
    par_solve(input)
}

pub fn par_dfs_solve(input: Input) -> bool {
    use rayon::prelude::*;

    let last = input.len() - 1;

    input
        .into_par_iter()
        .find_any(|(position, _)| *position == last)
        .is_some()
}

#[cfg(feature = "par_dfs")]
pub fn solve(input: Input) -> bool {
    par_dfs_solve(input)
}

pub fn par_dfs2_solve(input: Input) -> bool {
    use ahash::RandomState;
    use rayon::prelude::*;
    use std::cell::RefCell;
    use std::collections::HashSet;

    #[derive(Default)]
    struct IsVisited {}

    impl IsVisited {
        thread_local! {
            static LOCAL: RefCell<HashSet<State, RandomState>> =
                RefCell::new(HashSet::default());
        }

        pub fn is_marked(&self, state: &State) -> bool {
            Self::LOCAL.with_borrow(|local| local.contains(state))
        }

        pub fn mark(&self, state: State) -> bool {
            Self::LOCAL.with_borrow_mut(|local| local.insert(state))
        }
    }

    impl Drop for IsVisited {
        fn drop(&mut self) {
            Self::LOCAL.with_borrow_mut(|local| local.clear());
        }
    }

    fn solve<const P: usize>(
        mut to_visit: Vec<State>,
        has_stone: &[bool],
        is_visited: &IsVisited,
    ) -> Option<Vec<State>> {
        let len = has_stone.len();

        for _ in 0..P {
            match to_visit.pop() {
                None => break,

                Some(state @ (p, s)) => {
                    if p == len - 1 {
                        return None;
                    } else if is_visited.mark(state) {
                        let small_speed = s - 1;
                        let big_speed = s + 1;
                        let big_position = p + big_speed;

                        let next = Some((big_position, big_speed))
                            .into_iter()
                            .chain(
                                (small_speed > 0)
                                    .then_some((p + small_speed, small_speed)),
                            )
                            .chain(Some((p + s, s)))
                            .filter(|state @ &(p, _)| {
                                (p < len)
                                    && has_stone[p]
                                    && !is_visited.is_marked(state)
                            });

                        to_visit.extend(next);
                    }
                }
            }
        }

        Some(to_visit)
    }

    let max_task = rayon::current_num_threads();
    const P: usize = 3_096;

    let is_visited = IsVisited::default();
    let mut to_visit = vec![input.root];

    while !to_visit.is_empty() {
        let len = to_visit.len();

        if len < max_task {
            let next = solve::<P>(to_visit, input.has_stone, &is_visited);

            match next {
                Some(next) => to_visit = next,
                None => return true,
            }
        } else {
            let next = to_visit
                .par_drain(..)
                .chunks(len.div_ceil(max_task))
                .try_fold(Vec::new, |mut next, to_visit| {
                    let to_push =
                        solve::<P>(to_visit, input.has_stone, &is_visited)
                            .ok_or(())?;

                    next.extend(to_push);
                    Ok(next)
                })
                .try_reduce(Vec::new, |mut lhs, rhs| {
                    lhs.extend(rhs);
                    Ok(lhs)
                });

            match next {
                Ok(next) => to_visit.extend(next),
                Err(()) => return true,
            }
        }
    }

    false
}

#[cfg(feature = "par_dfs2")]
pub fn solve(input: Input) -> bool {
    par_dfs2_solve(input)
}

pub fn recursive_solve(input: Input) -> bool {
    use std::collections::HashMap;

    fn phi(
        root @ (p, s): State,
        has_stone: &[bool],
        is_visited: &mut HashMap<State, bool>,
    ) -> bool {
        if !is_visited.contains_key(&root) {
            let len = has_stone.len();

            let small_speed = s - 1;
            let big_speed = s + 1;
            let big_position = p + big_speed;

            let is_solution = (p == len - 1)
                || Some((p + s, s))
                    .into_iter()
                    .chain(
                        (small_speed > 0)
                            .then_some((p + small_speed, small_speed)),
                    )
                    .chain(Some((big_position, big_speed)))
                    .filter(|&(p, _)| (p < len) && has_stone[p])
                    .find(|&root| phi(root, has_stone, is_visited))
                    .is_some();

            is_visited.insert(root, is_solution);
        }

        is_visited.get(&root).copied().unwrap_or(false)
    }

    let mut is_visited = HashMap::default();
    phi(input.root, input.has_stone, &mut is_visited)
}

#[cfg(feature = "recursive")]
pub fn solve(input: Input) -> bool {
    recursive_solve(input)
}

pub fn walk_tree_solve(input: Input) -> bool {
    use dashmap::DashSet;
    use rayon::iter::walk_tree;
    use rayon::prelude::*;

    let is_visited = DashSet::new();

    walk_tree((0, 1), |&state @ (p, s)| {
        is_visited
            .insert(state)
            .then(|| {
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
                    .filter(|state @ &(p, _)| {
                        !is_visited.contains(state)
                            && (p < input.len())
                            && input.has_stone[p]
                    })
            })
            .into_iter()
            .flatten()
    })
    .find_any(|(position, _)| *position == input.len() - 1)
    .is_some()
}

#[cfg(feature = "walk_tree")]
pub fn solve(input: Input) -> bool {
    walk_tree_solve(input)
}
