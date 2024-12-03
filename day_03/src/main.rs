use regex::Regex;

fn part_one(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [left, right])| {
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .fold(0, |acc, (l, r)| acc + l * r)
}

fn part_two(input: &str) -> usize {

    let mut splits = input.split("don't()");

    let first = part_one(splits.next().unwrap());

    splits.fold(first, |acc, donts| {
        if let Some((_, dos)) = donts.split_once("do()") {
            acc + part_one(dos)
        } else {
            acc
        }
    })
}

fn main() {
    let input = include_str!("../input");
    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    static INPUT_PART_2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 161);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT_PART_2), 48);
    }
}
