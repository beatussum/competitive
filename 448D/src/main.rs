use std::cmp::{max, min};
use std::io::BufRead;

fn parse(input: &str) -> (usize, usize, usize) {
    let mut line = input.split_ascii_whitespace();

    (
        line.next().unwrap().parse().unwrap(),
        line.next().unwrap().parse().unwrap(),
        line.next().unwrap().parse().unwrap(),
    )
}

fn solve(m: usize, n: usize, k: usize) -> usize {
    let mut range = 1..(m * n + 1);
    let mut ret = 0;

    while !range.is_empty() {
        let mid = (range.start + range.end - 1) / 2;
        let count = (1..=n).map(|i| min((mid - 1) / i, m)).sum::<usize>();

        if count < k {
            ret = max(mid, ret);
            range.start = mid + 1;
        } else {
            range.end = mid;
        }
    }

    ret
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap();

    let (m, n, k) = parse(&input);
    println!("{}", solve(m, n, k));
}
