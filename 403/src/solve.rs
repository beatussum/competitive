use crate::Input;

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
