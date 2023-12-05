use std::iter::Peekable;

//const INPUT1: &'static str = include_str!("example1.txt");
const INPUT1: &'static str = include_str!("input1.txt");

fn parse_char<'a, I: Iterator<Item=&'a u8>>(v: u8) -> impl Fn(Peekable<I>) -> (Peekable<I>, Option<bool>) + 'a {
    move |mut i: Peekable<I>| {
        let matched = i.next_if(|&&c| c == v).is_some();
        (i, Some(matched))
    }
}

fn exact<'a, I: Iterator<Item=&'a u8>>(s: &'a [u8]) -> impl Fn(Peekable<I>) -> (Peekable<I>, Option<bool>) + 'a {
    move |mut i: Peekable<I>| {
        let mut expected = s.iter();
        while i.next_if(|&&c| expected.next().map(|&e| e == c).is_some()).is_some() {}
        (i, Some(expected.len() == 0))
    }
}

fn lex<'a, I: Iterator<Item=&'a u8>, T>(p: impl Fn(Peekable<I>) -> (Peekable<I>, Option<T>)) -> impl Fn(Peekable<I>) -> (Peekable<I>, Option<T>) {
    move |mut i: Peekable<I>| {
        while i.next_if(|&&c| c.is_ascii_whitespace()).is_some() {}
        p(i)
    }
}

fn parse_u64<'a, I: Iterator<Item=&'a u8>>(mut i: Peekable<I>) -> (Peekable<I>, Option<u64>) {
    let mut n = 0;
    while let Some(d) = i.next_if(|&&c| c.is_ascii_digit()) {
        n = n * 10 + (d - b'0') as u64;
    }
    (i, Some(n))
}

fn one_of<'a, 'b: 'a, I: Iterator<Item=&'a u8>>(opts: &'a [&'b [u8]]) -> impl Fn(Peekable<I>) -> (Peekable<I>, Option<usize>) + 'a {
    move |mut i: Peekable<I>| {
        let iters = opts.iter().map(|opt| opt.iter().peekable());
        let mut matched = None;
        for (ix, mut iter) in iters.enumerate() {
            while i.next_if(|&&c| iter.next_if(|&&e| e == c).is_some()).is_some() {}
            if iter.peek().is_none() {
                matched = Some(ix);
                break;
            }
        }
        (i, Some(matched.unwrap()))
    }
}

fn sep_by<'a, I: Iterator<Item=&'a u8>, T>(sep: u8, p: impl Fn(Peekable<I>) -> (Peekable<I>, Option<T>)) -> impl Fn(Peekable<I>) -> (Peekable<I>, Option<Vec<T>>) {
    move |i: Peekable<I>| {
        let mut items = vec![];

        let (mut i, r0) = p(i);
        if let Some(item) = r0 {
            items.push(item);
        } else  {
            return (i, None);
        }

        while i.next_if(|&&c| c == sep).is_some() {
            let (new_iter, r) = p(i);
            i = new_iter;
            if let Some(item) = r {
                items.push(item);
            }
        }

        (i, Some(items))
    }
}

fn parse_game<'a, I: Iterator<Item=&'a u8>>(i: Peekable<I>) -> (Peekable<I>, Option<(u64, Vec<Vec<(u64, usize)>>)>) {
    let (i, _) = exact(b"Game")(i);
    let (i, Some(id)) = lex(parse_u64)(i) else { panic!("bad input: failed to parse game id") };
    let (i, _) = parse_char(b':')(i);
    let (i, Some(ball_counts)) = sep_by(
        b';',
        sep_by(
            b',',
            |i| {
                const OPTS: [&'static [u8]; 3] = [b"blue", b"red", b"green"];

                let (i, Some(num_balls)) = lex(parse_u64)(i) else { panic!("bad input: failed to parse number of balls") };
                let (i, Some(ball_color_ix)) = lex(one_of(&OPTS))(i) else { panic!("bad input: failed to parse ball color") };
                (i, Some((num_balls, ball_color_ix)))
            }
        )
    )(i) else { panic!("bad input: failed to parse trial") };

    (i, Some((id, ball_counts)))
}

fn part1() {
    let actual_ball_counts = [14, 12, 13u64];
    let valid_id_sum = INPUT1.lines().fold(0, |valid_id_sum, line| {
        let i = line.as_bytes().iter().peekable();
        let (mut i, Some((id, ball_counts))) = parse_game(i) else { panic!("bad input: failed to parse game") };
        let is_invalid = ball_counts.iter().any(|trial| trial.iter().any(|&(num_balls, ball_color_ix)| num_balls > actual_ball_counts[ball_color_ix]));

        if let Some(&&c) = i.peek() {
            if c != b'\n' {
                panic!("bad input: non-newline trailing characters");
            }
        }

        valid_id_sum + if is_invalid { 0 } else { id }
    });
    println!("{}", valid_id_sum);
}

fn part2() {
    let sum = INPUT1.lines().fold(0, |sum, line| {
        let i = line.as_bytes().iter().peekable();
        let (mut i, Some((_, ball_counts))) = parse_game(i) else { panic!("bad input: failed to parse game") };
        let mins = ball_counts.iter().fold([0, 0, 0], |mins, trial| {
            trial.iter().fold(mins, |mut mins, &(num_balls, ball_color_ix)| {
                mins[ball_color_ix] = mins[ball_color_ix].max(num_balls);
                mins
            })
        });

        if let Some(&&c) = i.peek() {
            if c != b'\n' {
                panic!("bad input: non-newline trailing characters");
            }
        }

        sum + mins[0] * mins[1] * mins[2]
    });
    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
