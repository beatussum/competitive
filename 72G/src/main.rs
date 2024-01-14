use std::io;

fn fibonacci<const N: usize>(n: usize, memo: &mut [Option<usize>; N]) -> usize
{
    if memo[n].is_none() {
        memo[n] = Some(fibonacci(n - 1, memo) + fibonacci(n - 2, memo));
    }

    return memo[n].unwrap();
}

fn resolve(n: usize) -> usize
{
    const N: usize = 21;
    let mut memo: [Option<usize>; N] = [None; N];

    memo[0] = Some(1);
    memo[1] = Some(1);

    return fibonacci(n, &mut memo);
}

fn main()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("{}", resolve(input.trim().parse::<usize>().unwrap()))
}
