use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

fn get_antennas(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (i, line) in input.lines().rev().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                if let Some(antinodes) = antennas.get_mut(&c) {
                    antinodes.push((j as isize, i as isize));
                } else {
                    antennas.insert(c, vec![(j as isize, i as isize)]);
                }
            }
        }
    }

    antennas
}

fn part_one(input: &str) -> usize {
    let mut antinodes = HashSet::new();

    let max_y = input.lines().count() as isize;
    let max_x = input.chars().take_while(|c| *c != '\n').count() as isize;

    let antennas = get_antennas(input);

    antennas.into_values().for_each(|a_group| {
        a_group.iter().combinations(2).for_each(|ant| {
            let (x1, y1) = ant[0];
            let (x2, y2) = ant[1];

            let x_diff = (x1 - x2).abs();
            let y_diff = (y1 - y2).abs();

            let up_x = x_diff + max(x1, x2);
            let up_y = y_diff + max(y1, y2);
            let down_x = min(x1, x2) - x_diff;
            let down_y = min(y1, y2) - y_diff;

            if (x1 > x2 && y1 > y2) || (x2 > x1 && y2 > y1) {
                if up_x < max_x && up_y < max_y {
                    antinodes.insert((up_x, up_y));
                }

                if down_x >= 0 && down_y >= 0 {
                    antinodes.insert((down_x, down_y));
                }
            }

            if (x1 > x2 && y2 > y1) || (x2 > x1 && y1 > y2) {
                if down_x >= 0 && up_y < max_y {
                    antinodes.insert((down_x, up_y));
                }

                if up_x < max_x && down_y >= 0 {
                    antinodes.insert((up_x, down_y));
                }
            }
        });
    });

    antinodes.len()
}
fn part_two(input: &str) -> usize {
    let mut antinodes = HashSet::new();

    let max_y = input.lines().count() as isize;
    let max_x = input.chars().take_while(|c| *c != '\n').count() as isize;

    let antennas = get_antennas(input);

    antennas.into_values().filter(|v| v.len() >= 2).for_each(|a_group| {
        a_group.iter().combinations(2).for_each(|ant| {
            let (x1, y1) = ant[0];
            let (x2, y2) = ant[1];
            antinodes.insert((*x1, *y1));
            antinodes.insert((*x2, *y2));


            for another in 1..max_x {
                let x_diff = (x1 - x2).abs() * another;
                let y_diff = (y1 - y2).abs() * another;

                let up_x = x_diff + max(x1, x2);
                let up_y = y_diff + max(y1, y2);
                let down_x = min(x1, x2) - x_diff;
                let down_y = min(y1, y2) - y_diff;
                if (x1 > x2 && y1 > y2) || (x2 > x1 && y2 > y1) {
                    if up_x < max_x && up_y < max_y {
                        antinodes.insert((up_x, up_y));
                    }

                    if down_x >= 0 && down_y >= 0 {
                        antinodes.insert((down_x, down_y));
                    }
                }

                if (x1 > x2 && y2 > y1) || (x2 > x1 && y1 > y2) {
                    if down_x >= 0 && up_y < max_y {
                        antinodes.insert((down_x, up_y));
                    }

                    if up_x < max_x && down_y >= 0 {
                        antinodes.insert((up_x, down_y));
                    }
                }
            }
        });
    });

    antinodes.len()
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

#[cfg(test)]

mod tests {
    use crate::{part_one, part_two};

    static INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";


    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 14);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 34);
    }
}
