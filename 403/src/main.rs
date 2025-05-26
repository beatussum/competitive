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
fn solve(ps: &[bool]) -> bool {
    let mut dp = vec![vec![false; ps.len()]; ps.len()];

    for s in 0..ps.len() {
        dp.last_mut().unwrap()[s] = true;
    }

    for p in (0..(ps.len() - 1)).rev() {
        for s in 0..(ps.len() - p) {
            dp[p][s] = ps[p] && (dp[p + s][s] || dp[p + s + 1][s + 1]);
        }
    }

    dp[0][1]
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
