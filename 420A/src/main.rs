fn is_not_symetric_char(c: &char) -> bool
{
    match c {
        'A'|'H'|'I'|'M'|'O'|'T'|'U'|'V'|'W'|'X'|'Y' => false,
        _ => true
    }
}

fn resolve(chars: std::str::Chars) -> bool
{
    if chars.clone().find(is_not_symetric_char).is_none() {
        for (a, b) in chars.clone().zip(chars.rev()) {
            if a != b {
                return false;
            }
        }

        return true;
    } else {
        return false;
    }
}

fn main()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if resolve(input.trim().chars()) {
        println!("YES");
    } else {
        println!("NO");
    }
}
