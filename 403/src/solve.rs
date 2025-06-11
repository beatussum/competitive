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
                .filter(|all @ (p, _)| {
                    *p < len && !is_visited.contains(all) && input.has_stone[*p]
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
    #[cfg(feature = "par_dfs2_depth")]
    fn solve(
        mut next: Vec<State>,
        has_stone: &[bool],
        len: usize,
        is_visited: &HashSet<State>,
    ) -> Option<Vec<State>> {
        const MAX_DEPTH: usize = 256;

        let mut to_visit =
            next.drain(..).map(|state| (0, state)).collect::<Vec<_>>();

        while let Some((depth, state @ (p, s))) = to_visit.pop() {
            if p == len - 1 {
                return None;
            } else if depth > MAX_DEPTH {
                next.push(state);
            } else if is_visited.insert(state).is_ok() {
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
                    .filter(|state @ (position, _)| {
                        (*position < len)
                            && !is_visited.contains(state)
                            && has_stone[*position]
                    })
                    .map(|state| (depth + 1, state));

                to_visit.extend(next);
            }
        }

        next.extend(to_visit.into_iter().map(|(_, state)| state));
        Some(next)
    }

    use rayon::prelude::*;
    use scc::HashSet;

    let len = input.len();

    const MAX_TASK: usize = 8;
    let task_size: usize = rayon::current_num_threads();

    let is_visited = HashSet::new();
    let mut to_visit = vec![input.root];

    while !to_visit.is_empty() {
        let next = to_visit
            .par_drain(to_visit.len().saturating_sub(task_size * MAX_TASK)..)
            .chunks(task_size)
            .try_fold(Vec::new, |mut next, to_visit| {
                let to_push =
                    solve(to_visit, input.has_stone, len, &is_visited)
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

    false
}

#[cfg(feature = "par_dfs2")]
pub fn solve(input: Input) -> bool {
    par_dfs2_solve(input)
}

fn phi(position: usize, speed: usize, has_stone: &[bool]) -> bool {
    (position == has_stone.len() - 1)
        || (has_stone[position + speed]
            && phi(position + speed, speed, has_stone))
        || ((speed > 1)
            && has_stone[position + speed - 1]
            && phi(position + speed - 1, speed - 1, has_stone))
        || ((position + speed + 1 < has_stone.len())
            && has_stone[position + speed + 1]
            && phi(position + speed + 1, speed + 1, has_stone))
}

pub fn recursive_solve(input: Input) -> bool {
    let (p, s) = input.root;
    phi(p, s, input.has_stone)
}

#[cfg(feature = "recursive")]
pub fn solve(input: Input) -> bool {
    recursive_solve(input)
}

pub fn walk_tree_solve(input: Input) -> bool {
    use rayon::iter::walk_tree;
    use rayon::prelude::*;
    use scc::HashSet;

    let is_visited = HashSet::new();

    walk_tree((0, 1), |&state @ (p, s)| {
        let is_not_visited = is_visited.insert(state).is_ok();

        let small_speed = s - 1;
        let big_speed = s + 1;
        let big_position = p + big_speed;

        Some((big_position, big_speed))
            .into_iter()
            .chain((small_speed > 0).then_some((p + small_speed, small_speed)))
            .chain(Some((p + s, s)))
            .filter(move |(position, _)| {
                is_not_visited
                    && (*position < input.len())
                    && input.has_stone[*position]
            })
    })
    .find_any(|(position, _)| *position == input.len() - 1)
    .is_some()
}

#[cfg(feature = "walk_tree")]
pub fn solve(input: Input) -> bool {
    walk_tree_solve(input)
}
