const INPUT1: &'static str = include_str!("input1.txt");
//const INPUT1: &'static str = include_str!("example1.txt");

const INPUT2: &'static str = include_str!("input1.txt");
//const INPUT2: &'static str = include_str!("example2.txt");

struct Automaton<'a, const N: usize> {
    terms: [&'a [u8]; N],
    states: [u32; N],
}

impl<'a, const N: usize> Automaton<'a, N> {
    fn new(terms: [&'a [u8]; N]) -> Self {
        Self { states: [0; N], terms }
    }

    fn accept_first(&mut self, input_iter: &mut impl Iterator<Item=&'a u8>) -> Option<(usize, &'a [u8])> {
        while let Some(&c) = input_iter.next() {
            if let Some(ix) = self.step(c) {
                return Some((ix, self.terms[ix]));
            }
        }
        None
    }

    fn step(&mut self, c: u8) -> Option<usize> {
        let mut accept_ix = None;
        for (ix, (state, &term)) in self.states.iter_mut().zip(self.terms.iter()).enumerate() {
            if term[*state as usize] == c {
                *state += 1;
                if *state == term.len() as u32 {
                    *state = 0;
                    accept_ix = Some(ix);
                }
            } else {
                *state = (term[0] == c as u8) as u32;
            }
        }
        accept_ix
    }
}

fn part1() {
    let sum = INPUT1.lines().fold(0u64, |sum, line| {
        let mut l = line.as_bytes().iter();
        let first = l.find_map(|c| if char::is_digit(*c as char, 10) { Some(*c - '0' as u8) } else { None }).unwrap_or_else(|| 0) as u64;
        sum + match l.rfind(|&c| char::is_digit(*c as char, 10)).map(|c| *c - '0' as u8) {
            Some(last) => first * 10 + last as u64,
            None => first * 10 + first as u64,
        }
    });
    println!("{}", sum);
}

fn part2() {
    let terms = [
        b"0", b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9",
        b"zero",
        b"one",
        b"two",
        b"three",
        b"four",
        b"five",
        b"six",
        b"seven",
        b"eight",
        b"nine",
    ] as [&[u8]; 20];
    let rterms = [
        b"0", b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9",
        b"zero",
        b"eno",
        b"owt",
        b"eerht",
        b"ruof",
        b"evif",
        b"xis",
        b"neves",
        b"thgie",
        b"enin",
    ] as [&[u8]; 20];

    let sum = INPUT2.lines().fold(0u64, |sum, line| -> u64 {
        let mut automaton = Automaton::new(terms);
        let mut rautomaton = Automaton::new(rterms);

        let mut l = line.as_bytes().iter();
        let first = automaton.accept_first(&mut l).map(|(ix, _)| ix as u64 % 10 ).unwrap_or(0);
        let line_sum = match rautomaton.accept_first(&mut l.rev()) {
            Some((ix, _)) => first * 10 + ix as u64 % 10,
            None => first * 10 + first as u64,
        };

        line_sum + sum
    });
    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
