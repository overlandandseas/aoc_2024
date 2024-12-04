fn part_one(input: &str) -> usize {
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let max_y = rows.len();
    let max_x = rows[0].len();

    let mut answer = 0;
    for (i, column) in rows.iter().enumerate() {
        for (j, entry) in column.iter().enumerate() {
            if *entry == 'X' {
                answer += find_all_xmax(&rows, (j, i), (max_x, max_y));
            }
        }
    }
    answer
}

fn part_two(input: &str) -> usize {
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut answer = 0;
    for (i, column) in rows.iter().enumerate() {
        for (j, entry) in column.iter().enumerate() {
            if *entry == 'A' {
                if is_x(&rows, (j, i)) {
                    answer += 1;
                }
            }
        }
    }
    answer
}

fn is_x(rows: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    if let Some(first) = rows
        .get(y.overflowing_sub(1).0)
        .and_then(|col| col.get(x.overflowing_sub(1).0))
    {
        if let Some(last) = rows.get(y + 1).and_then(|col| col.get(x + 1)) {
            match (*first, *last) {
                ('M', 'S') => {}
                ('S', 'M') => {}
                _ => return false,
            };
        }
    }

    if let Some(first) = rows
        .get(y + 1)
        .and_then(|col| col.get(x.overflowing_sub(1).0))
    {
        if let Some(last) = rows
            .get(y.overflowing_sub(1).0)
            .and_then(|col| col.get(x + 1))
        {
            match (*first, (*last)) {
                ('M', 'S') => return true,
                ('S', 'M') => return true,
                _ => return false,
            };
        }
    }

    false
}

fn find_all_xmax(
    rows: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    (max_x, max_y): (usize, usize),
) -> usize {
    let mut number_of_xmas = 0;

    // RIGHT
    if x < max_x - 3 && is_mas(&rows, (x + 1, y), (x + 2, y), (x + 3, y)) {
        number_of_xmas += 1;
    }

    // LEFT
    if x >= 3 && is_mas(&rows, (x - 1, y), (x - 2, y), (x - 3, y)) {
        number_of_xmas += 1;
    }

    // DOWN
    if y < max_y - 3 && is_mas(&rows, (x, y + 1), (x, y + 2), (x, y + 3)) {
        number_of_xmas += 1;
    }

    // UP
    if y >= 3 && is_mas(&rows, (x, y - 1), (x, y - 2), (x, y - 3)) {
        number_of_xmas += 1;
    }

    //UP & to the RIGHT
    if x < max_x - 3 && y >= 3 && is_mas(&rows, (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)) {
        number_of_xmas += 1;
    }

    //DOWN & to the RIGHT
    if x < max_x - 3
        && y < max_y - 3
        && is_mas(&rows, (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3))
    {
        number_of_xmas += 1;
    }

    //DOWN & to the LEFT
    if x >= 3 && y < max_y - 3 && is_mas(&rows, (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)) {
        number_of_xmas += 1;
    }

    //UP & to the LEFT
    if x >= 3 && y >= 3 && is_mas(&rows, (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)) {
        number_of_xmas += 1;
    }

    number_of_xmas
}

fn is_mas(rows: &Vec<Vec<char>>, m: (usize, usize), a: (usize, usize), s: (usize, usize)) -> bool {
    rows[m.1][m.0] == 'M' && rows[a.1][a.0] == 'A' && rows[s.1][s.0] == 'S'
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    static INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 18);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 9);
    }
}
