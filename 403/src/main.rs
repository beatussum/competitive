fn parse<'a>(
    input: impl Iterator<Item = &'a str> + Clone,
) -> impl IntoIterator<Item = usize> + Clone {
    input.map(str::parse).map(Result::unwrap)
}

#[cfg(feature = "recursive")]
fn phi(s: usize, pos: usize, ps: &[bool]) -> bool {
    (pos == ps.len() - 1)
        || (ps[pos + s] && phi(s, pos + s, ps))
        || ((s > 1) && ps[pos + s - 1] && phi(s - 1, pos + s - 1, ps))
        || ((pos + s + 1 < ps.len())
            && ps[pos + s + 1]
            && phi(s + 1, pos + s + 1, ps))
}

#[cfg(feature = "recursive")]
fn solve(ps: &[bool]) -> bool {
    phi(1, 0, ps)
}

#[cfg(feature = "iterative")]
fn solve(positions: &[bool]) -> bool {
    use std::cell::Cell;

    let len = positions.len();

    let is_solvable_with = (0..len)
        .map(|p| {
            (0..len)
                .map(|_| Cell::new(p == len - 1))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    positions[..len - 1]
        .iter()
        .copied()
        .zip(is_solvable_with[..len - 1].iter())
        .enumerate()
        .rev()
        .filter_map(|(i, (p, speeds_for_p))| {
            Some((i, speeds_for_p)).filter(|_| p)
        })
        .for_each(|(p, speeds_for_p)| {
            speeds_for_p[..len - p].iter().enumerate().for_each(
                |(s, is_solvable)| {
                    is_solvable.set(
                        is_solvable_with[p + s][s].get()
                            || is_solvable_with[p + s + 1][s + 1].get(),
                    );
                },
            )
        });

    is_solvable_with[0][1].get()
}

#[cfg(feature = "par")]
fn solve(positions: &[bool]) -> bool {
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

    positions[..len - 1]
        .iter()
        .copied()
        .zip(is_solvable_with[..len - 1].iter())
        .enumerate()
        .rev()
        .filter_map(|(i, (p, speeds_of_p))| {
            Some((i, speeds_of_p)).filter(|_| p)
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

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().split(',');
    let parsed = parse(input);
    let last = parsed.clone().into_iter().last().unwrap();
    let mut input = vec![false; last + 1];

    for stone in parsed {
        input[stone] = true;
    }

    println!("{}", solve(&input));
}
