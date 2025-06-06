use frog_jump::{Input, parse_input, solve};

fn parse<'a>(
    input: impl Iterator<Item = &'a str> + Clone,
) -> impl IntoIterator<Item = usize> + Clone {
    input.map(str::parse).map(Result::unwrap)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().split(',');
    let input = parse(input);
    let has_stone = parse_input(input);
    let input = Input::new(&has_stone, (0, 1));
    println!("{}", solve(input));
}
