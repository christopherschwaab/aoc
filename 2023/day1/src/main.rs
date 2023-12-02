const INPUT: &'static str = include_str!("input.txt");
//const INPUT: &'static str = include_str!("example.txt");

//fn acceptor([]) {
//}

fn main() {
    let sum = INPUT.lines().fold(0u64, |sum, line| {
        let mut l = line.as_bytes().iter();
        let first = l.find_map(|c| if char::is_digit(*c as char, 10) { Some(*c - '0' as u8) } else { None }).unwrap_or_else(|| 0) as u64;
        sum + match l.rfind(|&c| char::is_digit(*c as char, 10)).map(|c| *c - '0' as u8) {
            Some(last) => first * 10 + last as u64,
            None => first * 10 + first as u64,
        }
    });
    println!("{}", sum);
}
