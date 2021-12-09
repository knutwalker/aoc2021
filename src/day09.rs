use aoc2021::{lines, PuzzleInput};
use derive_more::Deref;
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
    hm.low_points().map(|(_, _, h)| u64::from(h + 1)).sum()
}

fn part2(hm: &HeightMap) -> u64 {
    hm.low_points()
        .map(|(row, col, low)| {
            std::iter::from_fn({
                let mut seen = FxHashSet::with_capacity_and_hasher(64, FxBuildHasher::default());
                let mut queue = VecDeque::with_capacity(64);
                queue.push_back((row, col));

                move || loop {
                    let (r, c) = queue.pop_front()?;
                    if seen.insert((r, c)) {
                        queue.extend(hm.neighbors(r, c).filter_map(|(r, c, nh)| {
                            ((low..9).contains(&nh) && !seen.contains(&(r, c))).then(|| (r, c))
                        }));
                        break Some((r, c));
                    }
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

#[derive(Clone, Debug, Deref)]
pub struct HeightMap {
    rows: usize,
    cols: usize,
    #[deref]
    heights: Vec<Vec<u8>>,
}

impl HeightMap {
    fn iter(&self) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        (0..self.rows)
            .flat_map(move |row| (0..self.cols).map(move |col| (row, col, self[row][col])))
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        let cols = (col.saturating_sub(1)..self.cols.min(col + 2)).map(move |c| (row, c));
        let rows = (row.saturating_sub(1)..self.rows.min(row + 2)).map(move |r| (r, col));
        cols.chain(rows)
            .filter(move |&(r, c)| (r != row || c != col))
            .map(|(r, c)| (r, c, self[r][c]))
    }

    fn low_points(&self) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        self.iter()
            .filter(|&(row, col, h)| self.neighbors(row, col).all(|(r, c, nh)| h < nh))
    }
}

impl PuzzleInput for HeightMap {
    type Out = Self;

    fn from_input(input: &str) -> Self::Out {
        let heights = lines(input)
            .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self {
            rows: heights.len(),
            cols: heights[0].len(),
            heights,
        }
    }
}

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
