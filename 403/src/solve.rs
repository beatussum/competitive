pub fn dfs_solve(positions: &[bool]) -> bool {
    use std::collections::HashSet;

    let len = positions.len();

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
                .filter(|all @ (next_p, _)| {
                    *next_p < len
                        && !is_visited.contains(all)
                        && positions[*next_p]
                });

            stack.extend(to_visit)
        }
    }

    !stack.is_empty()
}

#[cfg(feature = "dfs")]
pub fn solve(positions: &[bool]) -> bool {
    dfs_solve(positions)
}

pub fn iterative_solve(positions: &[bool]) -> bool {
    let len = positions.len();

    let mut is_solvable_with = (0..len)
        .map(|p| (0..len).map(|_| p == len - 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    positions
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
pub fn solve(positions: &[bool]) -> bool {
    iterative_solve(positions)
}

pub fn par_solve(positions: &[bool]) -> bool {
    use rayon::prelude::*;

    let len = positions.len();

    let mut is_solvable_with = (0..len)
        .map(|p| (0..len).map(|_| p == len - 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    positions
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
pub fn solve(positions: &[bool]) -> bool {
    par_solve(positions)
}

pub fn par_dfs_solve(positions: &[bool]) -> bool {
    use crate::iterators::StateIterator;
    use rayon::iter::ParallelIterator;

    StateIterator::new((0, 1), positions)
        .find_any(|(position, _)| *position == positions.len() - 1)
        .is_some()
}

#[cfg(feature = "par_dfs")]
pub fn solve(positions: &[bool]) -> bool {
    par_dfs_solve(positions)
}

fn phi(speed: usize, position: usize, positions: &[bool]) -> bool {
    (position == positions.len() - 1)
        || (positions[position + speed]
            && phi(speed, position + speed, positions))
        || ((speed > 1)
            && positions[position + speed - 1]
            && phi(speed - 1, position + speed - 1, positions))
        || ((position + speed + 1 < positions.len())
            && positions[position + speed + 1]
            && phi(speed + 1, position + speed + 1, positions))
}

pub fn recursive_solve(positions: &[bool]) -> bool {
    phi(1, 0, positions)
}

#[cfg(feature = "recursive")]
pub fn solve(positions: &[bool]) -> bool {
    recursive_solve(positions)
}
