type ReportType = i32;
type Reports = std::collections::LinkedList<ReportType>;

fn resolve(mut r: Reports) -> Vec<usize>
{
    let mut ret: Vec<usize> = vec!(0);
    let mut counter = 0u8;

    while !r.is_empty() {
        if r.pop_front().unwrap() < 0 {
            if counter == 2 {
                counter = 1;
                ret.push(0);
            } else {
                counter += 1;
            }
        }

        *ret.last_mut().unwrap() += 1;
    }

    return ret;
}

fn main()
{
    let r = resolve(
        std::io::stdin()
        .lines()
        .nth(1)
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
    );

    println!("{}", r.len());

    for a in r {
        print!("{} ", a);
    }

    println!();
}
