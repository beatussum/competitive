use std::cmp::min;

use rayon::iter::IntoParallelIterator;

use crate::{Input, State};

pub fn dfs_solve(input: Input) -> bool {
    use std::collections::HashSet;

    let len = input.len();

    let mut is_visited = HashSet::new();
    let mut stack = vec![(0, 1)];

    while let Some((p, s)) = stack.pop() {
        if p == len - 1 {
            break;
        }

        if !is_visited.contains(&(p, s)) {
            is_visited.insert((p, s));

            let slow_speed = s - 1;
            let large_speed = s + 1;
            let large_position = p + large_speed;

            let to_visit = Some((large_position, large_speed))
                .into_iter()
                .chain((slow_speed > 0).then_some((p + slow_speed, slow_speed)))
                .chain(Some((p + s, s)))
                .filter(|all @ (position, _)| {
                    *position < len
                        && !is_visited.contains(all)
                        && input.has_stone[*position]
                });

            stack.extend(to_visit)
        }
    }

    !stack.is_empty()
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
    fn __solve(
        input: Input,
        max_depth: usize,
        is_visited: &HashSet<State>,
        len: usize,
    ) -> Vec<State> {
        let mut to_visit = vec![input.root];

        for _ in 0..max_depth {
            if let Some(all @ (p, s)) = to_visit.pop() {
                if p == len - 1 {
                    return vec![all];
                } else if is_visited.insert(all).is_ok() {
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
                        .filter(|all @ (position, _)| {
                            *position < len
                                && !is_visited.contains(all)
                                && input.has_stone[*position]
                        });

                    to_visit.extend(next)
                }
            }
        }

        to_visit
    }

    use rayon::prelude::*;
    use scc::HashSet;

    let len = input.len();

    let max_depth = 5;
    let max_task = rayon::current_num_threads() - 1;

    let is_visited = HashSet::new();
    let mut to_visit = vec![input.root];

    while let Some(last @ (p, _)) = to_visit.pop() {
        if p == len - 1 {
            return true;
        } else {
            let last_next = __solve(
                Input::new(input.has_stone, last),
                max_depth,
                &is_visited,
                len,
            );

            let next = to_visit
                .par_drain(0..min(max_task, to_visit.len()))
                .map(|base| Input::new(input.has_stone, base))
                .flat_map(|input| __solve(input, max_depth, &is_visited, len))
                .collect::<Vec<_>>();

            to_visit.extend(next);
            to_visit.extend(last_next);
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
