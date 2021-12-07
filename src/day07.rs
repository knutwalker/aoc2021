register!(
    "input/day7.txt";
    run(input: String) -> u32 {
        let numbers = input[0].split(',').flat_map(str::parse::<i32>).collect::<Vec<_>>();
        (part1(&numbers), part2(&numbers))
    }
);

fn part1(items: &[i32]) -> u32 {
    solve(items, |n| n)
}

fn part2(items: &[i32]) -> u32 {
    solve(items, |n| (n * (n + 1)) / 2)
}

fn solve(items: &[i32], cost: impl Fn(u32) -> u32) -> u32 {
    let min = items.iter().copied().min().unwrap();
    let max = items.iter().copied().max().unwrap();
    (min..=max)
        .map(|align| {
            items
                .iter()
                .map(|&num| cost((num - align).unsigned_abs()))
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::Solution;

    #[test]
    fn test_ex() {
        let input = r#"16,1,2,0,4,2,7,1,2,14"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 37);
        assert_eq!(res2, 168);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 344535);
        assert_eq!(res2, 95581659);
    }
}
