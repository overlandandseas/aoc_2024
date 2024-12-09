fn part_one(input: &str) -> usize {
    let max_char = char::from_u32(114112).unwrap();
    let blocks = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let num = c.to_digit(10).unwrap() as usize;
            if i % 2 != 0 {
                max_char.to_string().repeat(num)
            } else {
                char::from_u32((i / 2) as u32)
                    .unwrap()
                    .to_string()
                    .repeat(num)
            }
        })
        .collect::<String>();
    let mut blocks_vec: Vec<char> = blocks.chars().collect();
    let free_space = blocks.chars().filter(|c| *c == max_char).count();
    let filled_space = blocks_vec.len() - free_space;

    blocks
        .chars()
        .take(filled_space)
        .map(|c| {
            if c == max_char {
                let mut popped = max_char;
                while popped == max_char {
                    popped = blocks_vec.pop().unwrap();
                }
                popped
            } else {
                c
            }
        })
        .enumerate()
        .map(|(index, c)| index * (c as u32) as usize)
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
    // 90428939430 too low
    // 90428939430
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    static INPUT: &str = r"2333133121414131402";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 1928);
    }
}
