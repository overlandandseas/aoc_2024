use std::isize;

fn part_one(input: &str) -> isize {
    let lines = input.lines();
    lines.fold(0, |acc, line| {
        if is_line_safe(line, None) {
            acc + 1
        } else {
            acc
        }
    })
}

fn part_two(input: &str) -> isize {
    let lines = input.lines();

    lines.fold(0, |acc, line| {
        if is_line_safe(line, None) {
            acc + 1
        } else if (0..10).find(|n| is_line_safe(line, Some(*n))) != None {
            acc + 1
        } else {
            acc
        }
    })
}

fn is_line_safe(line: &str, skip: Option<usize>) -> bool {
    let tokens = line.split_whitespace();
    let mut numbers = tokens.clone().map(|val| val.parse::<isize>().unwrap());

    let mut first = numbers.next().unwrap();
    if skip == Some(0) {
        first = numbers.next().unwrap();
    }

    let mut prev = numbers.next().unwrap();
    if skip == Some(1) {
        prev = numbers.next().unwrap();
    }

    let increasing = first < prev;
    if !safe(increasing, first, prev) {
        return false;
    }

    let mut index = 2;
    while let Some(next) = numbers.next() {
        if skip == Some(index) {
            index += 1;
            continue;
        }
        if !safe(increasing, prev, next) {
            return false;
        }

        index += 1;
        prev = next;
    }

    true
}

fn safe(increasing: bool, prev: isize, next: isize) -> bool {
    if prev == next {
        return false;
    } else if increasing && (prev > next || next - prev > 3) {
        return false;
    } else if !increasing && (next > prev || prev - next > 3) {
        return false;
    }

    true
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
    println!("Part Tweo: {}", part_two(input));
}
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"6 6 4 2 1  
1 8 6 4 2 1
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 2);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 6);
    }
}
