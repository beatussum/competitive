use frog_jump::solve;

fn parse<'a>(
    input: impl Iterator<Item = &'a str> + Clone,
) -> impl IntoIterator<Item = usize> + Clone {
    input.map(str::parse).map(Result::unwrap)
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
