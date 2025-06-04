use frog_jump::{Input, solve};

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
    let mut has_stone = vec![false; last + 1];

    for p in parsed {
        has_stone[p] = true;
    }

    let input = Input::new(&has_stone, (0, 1));
    println!("{}", solve(input));
}
