fn parse<'a>(mut input: impl Iterator<Item = &'a str>) -> (u64, u64, u64) {
    (
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
    )
}

fn divides(a: u64, b: u64) -> u64 {
    if (a % b) == 0 {
        a / b
    } else {
        (a / b) + 1
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.split_whitespace();

    let (n, m, a) = parse(input);
    println!("{}", divides(n, a) * divides(m, a));
}
