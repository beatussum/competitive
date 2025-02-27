use std::io::BufRead;

fn solve(mut input: u32) -> u32 {
    let mut ret = 0;

    while input != 0 {
        ret += input & 1;
        input >>= 1;
    }

    ret
}

fn main() {
    let parsed: u32 = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    println!("{}", solve(parsed));
}
