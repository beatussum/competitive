fn parse<'a>(
    input: impl Iterator<Item = &'a str> + Clone,
) -> impl IntoIterator<Item = usize> + Clone {
    input.map(str::parse).map(Result::unwrap)
}

fn phi(s: usize, pos: usize, ps: &[bool]) -> bool {
    (pos == ps.len() - 1)
        || (ps[pos + s] && phi(s, pos + s, ps))
        || ((s > 1) && ps[pos + s - 1] && phi(s - 1, pos + s - 1, ps))
        || ((pos + s + 1 < ps.len()) && ps[pos + s + 1] && phi(s + 1, pos + s + 1, ps))
}

fn solve(ps: &[bool]) -> bool {
    phi(1, 0, ps)
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
