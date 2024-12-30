use std::{collections::HashMap, io::BufRead};

fn parse(mut input: impl Iterator<Item = String>) -> Vec<Vec<u8>>
{
    let mut ret = Vec::new();

    let t = input.next().unwrap().parse().unwrap();
    ret.reserve(t);

    for _ in 0..t {
        let mut case = Vec::new();

        let n = input.next().unwrap().parse().unwrap();
        case.reserve(n);

        case.extend(
            input
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10))
                .map(Option::unwrap)
                .map(|d| d as u8)
        );

        ret.push(case);
    }

    ret
}

fn solve(input: &[u8]) -> usize
{
    let mut sum = vec! [ 0 ];

    for i in input.iter().copied() {
        sum.push(sum.last().unwrap() + (i as isize));
    }

    let mut ret = 0;
    let mut map = HashMap::new();

    for (i, s) in sum.iter().copied().enumerate() {
        let value = map.entry(s - (i as isize)).or_insert(0);

        ret += *value;
        *value += 1;
    }

    ret
}

fn main()
{
    let input = std::io::stdin().lock().lines().map(Result::unwrap);
    let parsed = parse(input);

    for case in parsed.iter() {
        println!("{}", solve(case));
    }
}
