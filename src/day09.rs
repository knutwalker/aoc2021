use aoc2021::{lines, PuzzleInput};
use derive_more::Add;
use fxhash::{FxBuildHasher, FxHashSet};
use std::{cmp::Reverse, collections::VecDeque};
use tap::Tap;

register!(
    "input/day9.txt";
    (input: verbatim HeightMap) -> u64 {
        part1(&input), part2(&input)
    }
);

fn part1(hm: &HeightMap) -> u64 {
    hm.low_points().map(|(_, h)| u64::from(h + 1)).sum()
}

fn part2(hm: &HeightMap) -> u64 {
    let mut seen = FxHashSet::with_capacity_and_hasher(64, FxBuildHasher::default());
    let mut queue = VecDeque::with_capacity(64);

    hm.low_points()
        .map(|(pos, low)| {
            seen.clear();
            queue.clear();
            queue.push_back(pos);
            std::iter::from_fn(|| loop {
                let pos = queue.pop_front()?;
                if seen.insert(pos) {
                    queue.extend(hm.neighbors(pos).filter_map(|(pos, nh)| {
                        ((low..9).contains(&nh) && !seen.contains(&pos)).then(|| pos)
                    }));
                    break Some(pos);
                }
            })
            .count() as _
        })
        .collect::<Vec<_>>()
        .tap_mut(|sizes| sizes.sort_by_key(|&k| Reverse(k)))
        .iter()
        .take(3)
        .product()
}

#[derive(Clone, Debug)]
pub struct HeightMap(Box<[Box<[u8]>]>);

impl HeightMap {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn low_points(&self) -> impl Iterator<Item = (Pos, u8)> + '_ {
        self.0.iter().enumerate().flat_map(move |(row, hs)| {
            hs.iter().enumerate().filter_map(move |(col, &h)| {
                let pos = Pos(row as _, col as _);
                self.neighbors(pos).all(|(_, nh)| h < nh).then(|| (pos, h))
            })
        })
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = (Pos, u8)> + '_ {
        const NEIGHBORS: [Pos; 4] = [Pos(1, 0), Pos(-1, 0), Pos(0, 1), Pos(0, -1)];
        NEIGHBORS
            .iter()
            .map(move |&d| pos + d)
            .filter_map(|p @ Pos(r, c)| Some((p, *self.0.get(r as usize)?.get(c as usize)?)))
    }
}

impl PuzzleInput for HeightMap {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        Self(
            lines(input)
                .map(|line| {
                    line.bytes()
                        .map(|b| b - b'0')
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Add)]
struct Pos(i16, i16);

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::Solution;

    #[test]
    fn test_ex() {
        let input = r#"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        "#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 15);
        assert_eq!(res2, 1134);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 564);
        assert_eq!(res2, 1038240);
    }
}
