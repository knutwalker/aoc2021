#![feature(array_windows)]
#![allow(unused)]

#[macro_use]
extern crate aoc2021;

use aoc2021::Solution;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    for day in std::env::args()
        .skip(1)
        .filter_map(|s| s.parse::<u8>().ok())
    {
        let (res1, res2) = match day {
            1 => day01::Solver::solve(),
            2 => day02::Solver::solve(),
            3 => day03::Solver::solve(),
            4 => day04::Solver::solve(),
            5 => day05::Solver::solve(),
            6 => day06::Solver::solve(),
            7 => day07::Solver::solve(),
            8 => day08::Solver::solve(),
            9 => day09::Solver::solve(),
            10 => day10::Solver::solve(),
            11 => day11::Solver::solve(),
            12 => day12::Solver::solve(),
            13 => day13::Solver::solve(),
            14 => day14::Solver::solve(),
            15 => day15::Solver::solve(),
            16 => day16::Solver::solve(),
            17 => day17::Solver::solve(),
            18 => day18::Solver::solve(),
            19 => day19::Solver::solve(),
            20 => day20::Solver::solve(),
            21 => day21::Solver::solve(),
            22 => day22::Solver::solve(),
            23 => day23::Solver::solve(),
            24 => day24::Solver::solve(),
            25 => day25::Solver::solve(),
            x => unimplemented!("Day {} is not yet implemented", x),
        };

        println!("Day {:02} Part 1:\t{}", day, res1);
        println!("Day {:02} Part 2:\t{}", day, res2);
    }
}
