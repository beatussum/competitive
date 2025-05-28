pub fn dfs_solve(positions: &[bool]) -> bool {
    let len = positions.len();
    let mut stack = vec![(0, 1)];

    while !stack.is_empty() && (stack.last().unwrap().0 != len - 1) {
        let (p, s) = stack.pop().unwrap();

        if (p + s + 1 < len) && positions[p + s + 1] {
            stack.push((p + s + 1, s + 1));
        }

        if (s > 1) && positions[p + s - 1] {
            stack.push((p + s - 1, s - 1));
        }

        if positions[p + s] {
            stack.push((p + s, s));
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
        .skip(1)
        .filter_map(|(i, has_stone)| Some(i).filter(|_| has_stone))
        .for_each(|p| {
            let (first_position, other_positions) =
                is_solvable_with[p..].split_first_mut().unwrap();

            first_position[..len - p]
                .iter_mut()
                .enumerate()
                .skip(1)
                .for_each(|(s, is_solvable)| {
                    *is_solvable =
                        other_positions[s - 1][s] || other_positions[s][s + 1];
                });
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
        .skip(1)
        .filter_map(|(i, has_stone)| Some(i).filter(|_| has_stone))
        .for_each(|p| {
            let (first_position, other_positions) =
                is_solvable_with[p..].split_first_mut().unwrap();

            first_position[..len - p]
                .par_iter_mut()
                .enumerate()
                .skip(1)
                .for_each(|(s, is_solvable)| {
                    *is_solvable =
                        other_positions[s - 1][s] || other_positions[s][s + 1];
                });
        });

    is_solvable_with[0][1]
}

#[cfg(feature = "par")]
pub fn solve(positions: &[bool]) -> bool {
    par_solve(positions)
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
