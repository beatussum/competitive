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
                .map(|s| s.parse::<u32>())
                .map(Result::unwrap)
        );

        ret.push(case);
    }

    ret
}

fn solve(mut input: Vec<u32>) -> usize
{
    input.sort();

    let mut v = vec! [ (0, 0) ];
    v.reserve(input.len() + 1);

    for i in input.iter().copied() {
        let to_push = {
            let last = v.last().copied().unwrap();

            if last.1 == 0 {
                (i, i - 1)
            } else if last.0 < i {
                (i, (i - last.0) + last.1 - 1)
            } else {
                (last.0, last.1 - 1)
            }
        };

        v.push(to_push);
    }

    v.iter().copied().filter(|(_, i)| *i == 0).count() - 1
}

fn main()
{
    let input = std::io::stdin().lock().lines().map(Result::unwrap);
    let parsed = parse(input);

    for p in parsed {
        println!("{}", solve(p));
    }
}
