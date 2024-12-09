use std::{cmp::min, collections::HashMap, io::{BufRead, Read}};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Status
{
    pub has_contest: bool,
    pub has_gym: bool
}

fn build() -> Vec<Status>
{
    let mut ret = Vec::new();

    let mut buffer = String::new();

    if std::io::stdin().lock().read_line(&mut buffer).is_ok() {
        ret.reserve(buffer.trim().parse().unwrap());
    }

    buffer.clear();

    if std::io::stdin().lock().read_line(&mut buffer).is_ok() {
        ret.extend(
            buffer
                .split_whitespace()
                .map(|item| item.parse::<u8>().unwrap())

                .map(
                    |item| {
                        Status {
                            has_contest: (item % 2) == 1,
                            has_gym: item >= 2
                        }
                    }
                )
        );
    }

    ret
}

fn phi<'a, 'b>(input: &'a [Status], can_contest: bool, can_gym: bool, memo: &'b mut HashMap<(&'a [Status], bool, bool), usize>) -> usize
    where 'a: 'b
{
    match memo.get(&(input, can_contest, can_gym)) {
        Some(ret) => *ret,

        None => {
            let last = input.last().unwrap();

            if input.len() == 1 {
                if (last.has_contest && can_contest) || (last.has_gym && can_gym) {
                    0
                } else {
                    1
                }
            } else {
                let mut cmp = Vec::new();

                if last.has_contest && can_contest {
                    cmp.push(phi(&input[0..(input.len() - 1)], false, true, memo));
                }

                if last.has_gym && can_gym {
                    cmp.push(phi(&input[0..(input.len() - 1)], true, false, memo));
                }

                cmp.push(1 + phi(&input[0..(input.len() - 1)], true, true, memo));

                let ret = cmp.iter().min().copied().unwrap();
                memo.insert((input, can_contest, can_gym), ret);
                ret
            }
        }
    }
}

fn solve(input: &[Status]) -> usize
    { phi(input, true, true, &mut HashMap::new()) }

fn main()
{
    let parsed = build();
    println!("{}", solve(&parsed));
}
