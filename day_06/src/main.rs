use std::collections::HashSet;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Guard {
    dir: Direction,
    pos: (isize, isize),
}

impl Guard {
    fn front(&self) -> (isize, isize) {
        match self.dir {
            Direction::N => (self.pos.0, self.pos.1 + 1),
            Direction::E => (self.pos.0 + 1, self.pos.1),
            Direction::S => (self.pos.0, self.pos.1 - 1),
            Direction::W => (self.pos.0 - 1, self.pos.1),
        }
    }

    fn hash(&self) -> (Direction, (isize, isize)) {
        (self.dir, self.pos)
    }

    fn ghost_step(&self, rigids: &HashSet<(isize, isize)>) -> Self {
        let mut ghost_guard = self.clone();
        ghost_guard.rotate();
        ghost_guard.step(&rigids, None);
        ghost_guard
    }

    fn rotate(&mut self) {
        self.dir = match self.dir {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn step(&mut self, rigids: &HashSet<(isize, isize)>, obstical: Option<(isize, isize)>) {
        let front = self.front();

        if rigids.contains(&front) {
            self.rotate();
            return;
        }

        if let Some(obst) = obstical {
            if obst == front {
                self.rotate();
                return;
            }
        }
        self.pos = front;
    }
}

fn part_one(input: &str) -> usize {
    let max_y = input.lines().count() as isize;
    let max_x = input.lines().next().unwrap().chars().count() as isize;

    let mut rigids = HashSet::new();

    let mut paces = HashSet::new();

    let mut guard = Guard {
        dir: Direction::N,
        pos: (0, 0),
    };

    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                rigids.insert((x as isize, y as isize));
            }
            if c == '^' {
                guard.pos = (x as isize, y as isize)
            }
        }
    }

    while inbounds(&guard, max_x, max_y) {
        paces.insert(guard.pos);
        guard.step(&rigids, None);
    }

    paces.len()
}

fn part_two(input: &str) -> usize {
    let max_y = input.lines().count() as isize;
    let max_x = input.lines().next().unwrap().chars().count() as isize;

    let mut rigids = HashSet::new();

    let mut guard = Guard {
        dir: Direction::N,
        pos: (0, 0),
    };

    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                rigids.insert((x as isize, y as isize));
            }
            if c == '^' {
                guard.pos = (x as isize, y as isize)
            }
        }
    }

    let mut obsticals = HashSet::new();
    let mut paces = HashSet::new();
    let mut paces_pos = HashSet::new();
    while inbounds(&guard, max_x, max_y) {
        paces.insert(guard.hash());
        paces_pos.insert(guard.pos);

        let obstical = guard.front();

        if !obsticals.contains(&obstical) && !rigids.contains(&obstical) && !paces_pos.contains(&obstical) {
            let mut ghost_paces = HashSet::new();
            let mut ghost_guard = guard.ghost_step(&rigids);
            while inbounds(&ghost_guard, max_x, max_y) {
                if paces.contains(&ghost_guard.hash()) || ghost_paces.contains(&ghost_guard.hash())
                {
                    obsticals.insert(obstical);
                    break;
                } else {
                    ghost_paces.insert(ghost_guard.hash());
                    ghost_guard.step(&rigids, Some(obstical));
                }
            }
        }

        guard.step(&rigids, None);
    }
    obsticals.len()
}

fn inbounds(guard: &Guard, x: isize, y: isize) -> bool {
    guard.pos.0 < x && guard.pos.1 < y && guard.pos.0 >= 0 && guard.pos.1 >= 0
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
    // 5780 too high
    // 1725 too high
    // 1722 too high
    // 1675 WRONG
    // 1657 WRONG
    // 1648 WRONG
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    static INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 41);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 6);
    }
}
