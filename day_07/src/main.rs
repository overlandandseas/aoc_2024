fn part_one(input: &str) -> u128 {
    let mut answer = 0;
    for line in input.lines() {
        let (test_value, params) = line.split_once(": ").unwrap();

        let value: u128 = test_value.parse().unwrap();

        let params_vec: Vec<u128> = params.split(" ").flat_map(|c| c.parse()).collect();

        if can_this_equal_that(&params_vec, value, false) {
            answer += value;
        }
    }

    answer
}

fn part_two(input: &str) -> u128 {
    input
        .lines()
        .map(|line| {
            let (test_value, params) = line.split_once(": ").unwrap();

            let value: u128 = test_value.parse().unwrap();

            let params_vec: Vec<u128> = params.split(" ").flat_map(|c| c.parse()).collect();

            if can_this_equal_that(&params_vec, value, true) {
                value
            } else {
                0
            }
        })
        .sum()
}

fn can_this_equal_that(params: &Vec<u128>, value: u128, use_str: bool) -> bool {
    if params.iter().sum::<u128>() == value {
        return true;
    }

    if params
        .iter()
        .fold(0, |acc, val| if acc == 0 { acc } else { acc * val })
        == value
    {
        return true;
    }

    if use_str && params.iter().map(|i| i.to_string()).collect::<String>() == value.to_string() {
        return true;
    }

    if let Some(first) = params.get(0) {
        if let Some(second) = params.get(1) {
            let rest_add = vec![vec![first + second], params[2..].to_vec()].concat();

            if can_this_equal_that(&rest_add, value, use_str) {
                return true;
            }

            let rest_mul = vec![vec![first * second], params[2..].to_vec()].concat();

            if can_this_equal_that(&rest_mul, value, use_str) {
                return true;
            }

            if use_str {
                let rest_str = format!(
                    "{}{}",
                    first,
                    second,
                );

                let rest_str_vec = vec![vec![rest_str.parse::<u128>().unwrap()], params[2..].to_vec()].concat();

                if can_this_equal_that(&rest_str_vec, value, true) {
                    return true;
                }
            }
        }
    }

    false
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    static INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 3749);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 11387);
    }
}
