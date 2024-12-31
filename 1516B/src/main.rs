use std::io::BufRead;

fn parse(mut input: impl Iterator<Item = String>) -> Vec<Vec<u32>>
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
                .split_ascii_whitespace()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
        );

        ret.push(case);
    }

    ret
}

fn solve(input: &[u32]) -> bool
{
    let mut xor = vec![ 0 ];
    xor.reserve(input.len() + 1);

    for i in input.iter().copied() {
        let to_push = xor.last().copied().unwrap() ^ i;
        xor.push(to_push);
    }

    let last = xor.last().copied().unwrap();

    for (i, a) in xor.iter().copied().enumerate() {
        for b in xor[i..].iter().copied().skip(1) {
            if (a == a ^ b) && (a == b ^ last) {
                return true;
            }
        }
    }

    false
}

fn main()
{
    let input = std::io::stdin().lock().lines().map(Result::unwrap);
    let parsed = parse(input);

    for case in parsed.iter() {
        println!("{}", if solve(case) { "YES" } else { "NO" });
    }
}
