use std::collections::HashMap;

fn part_one(input: &str) -> isize {
    let mut nums = input.split_whitespace();
    let mut left: Vec<isize> = vec![];
    let mut right = vec![];

    while let Some(num_str) = nums.next() {
        let left_num = num_str.parse().unwrap();
        left.push(left_num);

        let right_num = nums.next().unwrap().parse().unwrap();
        right.push(right_num);
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut nums = input.split_whitespace();
    let mut left = vec![];
    let mut right = HashMap::new();

    while let Some(num_str) = nums.next() {
        left.push(num_str);

        let right_num = nums.next().unwrap();
        if let Some(frequency) = right.get_mut(right_num) {
            *frequency += 1;
        } else {
            right.insert(right_num, 1);
        }
    }

    left.iter()
        .flat_map(|l| right.get(l).map(|r| l.parse::<usize>().unwrap() * r))
        .sum()
}

fn main() {
    let input = include_str!("../input");
    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 11);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 31);
    }
}
