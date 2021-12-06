register!(
    "input/day6.txt";
    run(input: String) -> usize {
        let numbers = input[0].split(',').flat_map(str::parse::<usize>).collect::<Vec<_>>();
        (part1(&numbers), part2(&numbers))
    }
);

fn part1(items: &[usize]) -> usize {
    model_fishes(items, 80)
}

fn part2(items: &[usize]) -> usize {
    model_fishes(items, 256)
}

fn model_fishes(initial: &[usize], days: usize) -> usize {
    let mut fishes = [0_usize; 9];

    for &timer in initial {
        fishes[timer] += 1;
    }

    for _ in 0..days {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::Solution;

    #[test]
    fn test_ex() {
        let input = r#"3,4,3,1,2"#;
        let (res1, res2) = Solver::run_on(input);
        assert_eq!(res1, 5934);
        assert_eq!(res2, 26984457539);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 360610);
        assert_eq!(res2, 1631629590423);
    }
}
