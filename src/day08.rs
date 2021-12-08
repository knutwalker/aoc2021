use const_combinations::IterExt;
use derive_more::Deref;
use parse_display::FromStr;
use std::{
    convert::Infallible,
    ops::{Index, IndexMut},
    str::FromStr,
};

register!(
    "input/day8.txt";
    run(input: parse Input) -> usize {
        (part1(&input), part2(&input))
    }
);

fn part1(items: &[Input]) -> usize {
    items
        .iter()
        .flat_map(|Input(_, output_digits)| output_digits.0)
        .flat_map(|d| d.guess())
        .count()
}

fn part2(items: &[Input]) -> usize {
    items
        .iter()
        .flat_map(|&Input(test, output)| {
            (b'a'..=b'g').permutations().map(Guess).find_map(|guess| {
                test.iter().all(|d| d.guess_for(guess).is_some()).then(|| {
                    output
                        .iter()
                        .flat_map(|d| d.guess_for(guess))
                        .fold(0_usize, |n, d| n * 10 + usize::from(d))
                })
            })
        })
        .sum()
}

#[derive(Clone, Copy, Debug, FromStr)]
#[display("{0} | {1}")]
pub struct Input(Displays<10>, Displays<4>);

#[derive(Clone, Copy, Debug, Deref)]
struct Displays<const N: usize>([Display; N]);

impl<const N: usize> FromStr for Displays<N> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Displays(
            s.split_ascii_whitespace()
                .flat_map(Display::from_str)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Display([bool; 7]);

impl Display {
    fn guess(self) -> Option<u8> {
        let size = self.0.iter().filter(|x| **x).count();
        match size {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    fn guess_for(self, guess: Guess) -> Option<u8> {
        let mut options = (0..=9).filter(|&n| guess.guess_for(n) == self);
        let first = options.next()?;
        match options.next() {
            Some(_ambiguous) => None,
            None => Some(first),
        }
    }
}

impl FromStr for Display {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.bytes().fold(
            Default::default(),
            |mut digits, digit| {
                digits[usize::from(digit - b'a')] = true;
                digits
            },
        )))
    }
}

impl Index<u8> for Display {
    type Output = bool;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[usize::from(index - b'a')]
    }
}

impl IndexMut<u8> for Display {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[usize::from(index - b'a')]
    }
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const E: usize = 4;
const F: usize = 5;
const G: usize = 6;

#[derive(Copy, Clone, Debug)]
struct Guess([u8; 7]);

impl Guess {
    fn guess_for(self, n: u8) -> Display {
        let mut display = Display::default();

        match n {
            0 => {
                display[self.0[A]] = true;
                display[self.0[B]] = true;
                display[self.0[C]] = true;
                display[self.0[E]] = true;
                display[self.0[F]] = true;
                display[self.0[G]] = true;
            }
            1 => {
                display[self.0[C]] = true;
                display[self.0[F]] = true;
            }
            2 => {
                display[self.0[A]] = true;
                display[self.0[C]] = true;
                display[self.0[D]] = true;
                display[self.0[E]] = true;
                display[self.0[G]] = true;
            }
            3 => {
                display[self.0[A]] = true;
                display[self.0[C]] = true;
                display[self.0[D]] = true;
                display[self.0[F]] = true;
                display[self.0[G]] = true;
            }
            4 => {
                display[self.0[B]] = true;
                display[self.0[C]] = true;
                display[self.0[D]] = true;
                display[self.0[F]] = true;
            }
            5 => {
                display[self.0[A]] = true;
                display[self.0[B]] = true;
                display[self.0[D]] = true;
                display[self.0[F]] = true;
                display[self.0[G]] = true;
            }
            6 => {
                display[self.0[A]] = true;
                display[self.0[B]] = true;
                display[self.0[D]] = true;
                display[self.0[E]] = true;
                display[self.0[F]] = true;
                display[self.0[G]] = true;
            }
            7 => {
                display[self.0[A]] = true;
                display[self.0[C]] = true;
                display[self.0[F]] = true;
            }
            8 => {
                display[self.0[A]] = true;
                display[self.0[B]] = true;
                display[self.0[C]] = true;
                display[self.0[D]] = true;
                display[self.0[E]] = true;
                display[self.0[F]] = true;
                display[self.0[G]] = true;
            }
            9 => {
                display[self.0[A]] = true;
                display[self.0[B]] = true;
                display[self.0[C]] = true;
                display[self.0[D]] = true;
                display[self.0[F]] = true;
                display[self.0[G]] = true;
            }
            _ => {}
        };

        display
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::Solution;

    #[test]
    fn test_small() {
        let input = r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 0);
        assert_eq!(res2, 5353);
    }

    #[test]
    fn test_ex() {
        let input = r#"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 26);
        assert_eq!(res2, 61229);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 534);
        assert_eq!(res2, 1070188);
    }
}
