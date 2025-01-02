use std::{collections::HashMap, io::BufRead};

fn parse(mut input: impl Iterator<Item = String>) -> Vec<u32>
{
    let mut ret = Vec::new();

    let t = input.next().unwrap().parse().unwrap();
    ret.reserve(t);

    for _ in 0..t {
        ret.push(input.next().unwrap().parse().unwrap());
    }

    ret
}

fn build(input: u32) -> HashMap<u32, usize>
{
    let mut f = HashMap::<_, _>::from_iter((1..=9).map(|i| (i, i as usize)));

    for i in 10..=input {
        f.insert(i, f.get(&(i / 10)).copied().unwrap() + ((i % 10) as usize));
    }

    let mut phi = HashMap::from([(1, 1)]);

    for i in 2..=input {
        phi.insert(
            i,
            phi.get(&(i - 1)).copied().unwrap() + f.get(&i).copied().unwrap()
        );
    }

    phi
}

fn main()
{
    let input = std::io::stdin().lock().lines().map(Result::unwrap);
    let parsed = parse(input);

    let built = build(parsed.iter().max().copied().unwrap());

    for p in parsed {
        println!("{}", built.get(&p).copied().unwrap());
    }
}
