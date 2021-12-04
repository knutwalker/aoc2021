use derive_more::{Deref, DerefMut};
use std::ops::ControlFlow;

register!(
    "input/day4.txt";
    run(input: chunk String) -> u32 {
        let (draws, mut boards) = parse(&input);
        (part1(&draws, &mut boards), part2(&draws, &mut boards))
    }
);

fn part1(draws: &[u8], boards: &mut Boards) -> u32 {
    draws
        .iter()
        .try_for_each(|num| boards.draw(*num))
        .break_value()
        .expect("no solution")
}

fn part2(draws: &[u8], boards: &mut Boards) -> u32 {
    draws
        .iter()
        .try_for_each(|num| boards.draw2(*num))
        .break_value()
        .expect("no solution")
}

fn parse(input: &[Vec<String>]) -> (Vec<u8>, Boards) {
    let (draws, boards) = input.split_first().unwrap();
    let draws = draws
        .iter()
        .flat_map(|d| d.split(','))
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .collect();

    let boards = Boards(
        boards
            .iter()
            .map(|lines| {
                Board(
                    lines
                        .iter()
                        .map(|line| {
                            line.split_ascii_whitespace()
                                .map(|s| (s.parse().unwrap(), false))
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                )
            })
            .collect(),
    );

    (draws, boards)
}

const BOARD_SIZE: usize = 5;

#[derive(Deref, DerefMut)]
struct Board([[(u8, bool); BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    fn draw(&mut self, number: u8) -> bool {
        for row in self.iter_mut() {
            for (num, drawn) in row {
                if *num == number {
                    *drawn = true;
                }
            }
        }
        self.has_won()
    }

    fn has_won(&self) -> bool {
        if self.iter().any(|row| row.iter().all(|(_, drawn)| *drawn)) {
            return true;
        }

        for c in 0..BOARD_SIZE {
            if self.iter().all(|row| row[c].1) {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u32 {
        self.iter()
            .flat_map(|row| row.iter())
            .filter_map(|(num, drawn)| (!*drawn).then(|| u32::from(*num)))
            .sum::<u32>()
    }
}

#[derive(Deref, DerefMut)]
struct Boards(Vec<Board>);

impl Boards {
    fn draw(&mut self, number: u8) -> ControlFlow<u32> {
        self.iter_mut()
            .find_map(|board| {
                board
                    .draw(number)
                    .then(|| ControlFlow::Break(board.score() * u32::from(number)))
            })
            .unwrap_or(ControlFlow::CONTINUE)
    }

    fn draw2(&mut self, number: u8) -> ControlFlow<u32> {
        let removed = self.drain_filter(|board| board.draw(number)).last();
        self.is_empty()
            .then(|| ControlFlow::Break(removed.unwrap().score() * u32::from(number)))
            .unwrap_or(ControlFlow::CONTINUE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::Solution;

    #[test]
    fn test_ex() {
        let input = r#"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 4512);
        assert_eq!(res2, 1924);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 32844);
        assert_eq!(res2, 4920);
    }
}
