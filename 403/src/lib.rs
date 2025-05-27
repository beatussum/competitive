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
    use std::sync::atomic::{AtomicBool, Ordering};

    let len = positions.len();

    let is_solvable_with = (0..len)
        .map(|p| {
            (0..len)
                .map(|_| AtomicBool::new(p == len - 1))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    positions
        .iter()
        .copied()
        .zip(&is_solvable_with)
        .enumerate()
        .rev()
        .skip(1)
        .filter_map(|(i, (b, speeds_of_p))| {
            Some((i, speeds_of_p)).filter(|_| b)
        })
        .for_each(|(p, speeds_of_p)| {
            speeds_of_p[..len - p].into_par_iter().enumerate().for_each(
                |(s, is_solvable)| {
                    is_solvable.store(
                        is_solvable_with[p + s][s].load(Ordering::Relaxed)
                            || is_solvable_with[p + s + 1][s + 1]
                                .load(Ordering::Relaxed),
                        Ordering::Relaxed,
                    );
                },
            )
        });

    is_solvable_with[0][1].load(Ordering::Relaxed)
}

#[cfg(feature = "par")]
pub fn solve(positions: &[bool]) -> bool {
    par_solve(positions)
}
