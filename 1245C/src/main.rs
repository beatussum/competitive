use std::io::BufRead;

fn solve(input: &str) -> usize
{
    fn is_invalid(c: char) -> bool
        { matches!(c, 'm' | 'w') }

    if input.len() == 1 {
        if is_invalid(input.chars().nth(0).unwrap()) {
            0
        } else {
            1
        }
    } else {
        let mut dp = vec! [0 ; input.len() + 1];

        dp[0] = 1;
        dp[1] = 1;

        let it = input.chars().zip(input.chars().skip(1)).enumerate();

        for (i, (a, b)) in it {
            if is_invalid(a) || is_invalid(b) {
                return 0;
            } else if (a == b) && matches!(a, 'n' | 'u') {
                dp[i + 2] = (dp[i + 1] + dp[i]) % 1_000_000_007;
            } else {
                dp[i + 2] = dp[i + 1];
            }
        }

        dp.last().copied().unwrap()
    }
}

fn main()
{
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input).unwrap();
    println!("{}", solve(input.trim()));
}
