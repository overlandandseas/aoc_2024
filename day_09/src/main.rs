static MAX_CHAR: char = '\u{10FFFF}';

fn get_blocks(input: &str) -> String {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let num = c.to_digit(10).unwrap() as usize;
            if i % 2 != 0 {
                MAX_CHAR.to_string().repeat(num)
            } else {
                char::from_u32((i / 2) as u32)
                    .unwrap()
                    .to_string()
                    .repeat(num)
            }
        })
        .collect::<String>()
}

fn part_one(input: &str) -> usize {
    let blocks = get_blocks(input);
    let mut blocks_vec: Vec<char> = blocks.chars().collect();
    let free_space = blocks.chars().filter(|c| *c == MAX_CHAR).count();
    let filled_space = blocks_vec.len() - free_space;

    blocks
        .chars()
        .take(filled_space)
        .map(|c| {
            if c == MAX_CHAR {
                let mut popped = MAX_CHAR;
                while popped == MAX_CHAR {
                    popped = blocks_vec.pop().unwrap();
                }
                popped
            } else {
                c
            }
        })
        .enumerate()
        .map(|(index, c)| index * (c as usize))
        .sum::<usize>()
}

#[derive(Clone, Debug)]
struct Mem {
    file: char,
    size: usize,
}

fn get_memories(blocks: &str) -> Vec<Mem> {
    let mut current_file = MAX_CHAR;
    let mut current_size = 0;

    let mut memories = vec![];
    for c in blocks.chars() {
        if c == MAX_CHAR {
            continue;
        }
        if c != current_file {
            memories.push(Mem {
                file: current_file,
                size: current_size,
            });
            current_size = 1;
            current_file = c;
        } else {
            current_size += 1;
        }
    }

    memories.push(Mem {
        file: current_file,
        size: current_size,
    });

    memories.reverse();

    memories
}

fn find_small_enough_memory(free_space_size: usize, memories: &mut Vec<Mem>) -> Vec<Mem> {
    let mut found_mem = vec![];
    let mut left = free_space_size;
    let mut removed = 0;

    for (index, mem) in memories.clone().iter().enumerate() {
        if mem.size <= left {
            found_mem.push(memories.remove(index - removed));
            removed += 1;
            left -= mem.size;
        }
    }

    found_mem
}

fn defrag(blocks: &str) -> String {
    let mut memories = get_memories(&blocks);

    let mut free_space_start = 0;
    let mut free_space_size = 0;
    let mut in_free = false;
    let mut blocks_mod = blocks.chars().collect::<Vec<char>>();
    for (index, c) in blocks.chars().enumerate() {
        if c == MAX_CHAR {
            if !in_free {
                free_space_start = index;
                in_free = true;
                free_space_size = 1;
            } else {
                free_space_size += 1;
            }
        } else {
            if in_free {
                let mems = find_small_enough_memory(free_space_size, &mut memories);
                for mem in mems {
                    if let Some(pos) = blocks.chars().position(|c| c == mem.file) {
                        if pos >= index {
                            blocks_mod = blocks_mod
                                .into_iter()
                                .map(|c| if c == mem.file { MAX_CHAR } else { c })
                                .collect::<Vec<_>>();
                            for i in free_space_start..free_space_start + mem.size {
                                blocks_mod[i] = mem.file;
                            }
                            free_space_start = free_space_start + mem.size;
                        }
                    }
                }
                in_free = false;
                // } else {
                //     if let Some(pos) = memories.iter().position(|mem| mem.file == c) {
                //         memories.remove(pos);
                //     }
            }
        }
    }

    blocks_mod.iter().collect::<String>()
}

fn part_two(input: &str) -> usize {
    let blocks = get_blocks(input);

    let blocks_mod = defrag(&blocks);

    blocks_mod
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != MAX_CHAR)
        .map(|(i, c)| i * c as usize)
        .sum()
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
    println!("Part Two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    static INPUT: &str = r"2333133121414131402";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 1928);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 2858);
    }
}
