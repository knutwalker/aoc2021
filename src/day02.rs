use parse_display::FromStr;

register!(
    "input/day2.txt";
    run(input: parse Command) -> i64 {
        (part1(&input), part2(&input))
    }
);

#[derive(Clone, Copy, Debug, FromStr)]
#[display(style = "lowercase")]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Clone, Copy, Debug, FromStr)]
#[display("{0} {1}")]
pub struct Command(Direction, i64);

fn part1(items: &[Command]) -> i64 {
    let (mut horizontal, mut depth) = (0, 0);
    for Command(direction, unit) in items {
        match direction {
            Direction::Forward => horizontal += unit,
            Direction::Down => depth += unit,
            Direction::Up => depth -= unit,
        }
    }
    horizontal * depth
}

fn part2(items: &[Command]) -> i64 {
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
    for Command(direction, unit) in items {
        match direction {
            Direction::Forward => {
                horizontal += unit;
                depth += aim * unit
            }
            Direction::Down => aim += unit,
            Direction::Up => aim -= unit,
        }
    }
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::Solution;

    #[test]
    fn test_ex() {
        let items = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
        let (res1, res2) = Solver::run_on(items);
        assert_eq!(res1, 150);
        assert_eq!(res2, 900);
    }

    #[test]
    fn test() {
        let (res1, res2) = Solver::run_on_input();
        assert_eq!(res1, 1698735);
        assert_eq!(res2, 1594785890);
    }
}
