use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
struct PageOrdering<'a> {
    not_allowed_before: HashSet<&'a str>,
    not_allowed_after: HashSet<&'a str>,
}

impl<'a> PageOrdering<'a> {
    fn new() -> Self {
        Self {
            not_allowed_before: HashSet::new(),
            not_allowed_after: HashSet::new(),
        }
    }

    fn safe<I>(&self, check: I, page: &str) -> bool
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut cannot_contain = &self.not_allowed_before;

        for item in check {
            if item == page {
                cannot_contain = &self.not_allowed_after;
                continue;
            }

            if cannot_contain.contains(item) {
                return false;
            }
        }

        true
    }
}

fn is_line_safe(rules: &HashMap<&str, PageOrdering>, line: &str) -> bool {
    let mut page_numbers = line.split(",");

    while let Some(page_number) = page_numbers.next() {
        if let Some(page) = rules.get(page_number) {
            if !page.safe(page_numbers.clone(), page_number) {
                return false;
            }
        }
    }

    true
}

fn make_rules(page_ordering_rules: &str) -> HashMap<&str, PageOrdering> {
    let mut rules: HashMap<&str, PageOrdering> = HashMap::new();
    for rule in page_ordering_rules.lines() {
        let (left, right) = rule.split_once("|").unwrap();

        if let Some(left_page) = rules.get_mut(left) {
            left_page.not_allowed_after.insert(right);
        } else {
            let mut page_ordering = PageOrdering::new();
            page_ordering.not_allowed_after.insert(right);
            rules.insert(left, page_ordering);
        }

        if let Some(right_page) = rules.get_mut(right) {
            right_page.not_allowed_before.insert(left);
        } else {
            let mut page_ordering = PageOrdering::new();
            page_ordering.not_allowed_before.insert(left);
            rules.insert(right, page_ordering);
        }
    }

    rules
}

fn part_one(input: &str) -> usize {
    let (page_ordering_rules, updates) = input.split_once("\n\n").unwrap();

    let rules = make_rules(page_ordering_rules);

    updates
        .lines()
        .filter(|s| is_line_safe(&rules, s))
        .flat_map(|update| {
            let pages: Vec<&str> = update.split(",").collect();

            pages
                .iter()
                .take((pages.len() / 2) + 1)
                .last()
                .unwrap()
                .parse::<usize>()
        })
        .sum()
}

fn make_line_safe<'a>(rules: &HashMap<&str, PageOrdering>, line: &'a str) -> Vec<&'a str> {
    let mut unsafe_vec: Vec<&str> = line.split(",").collect();

    unsafe_vec.sort_by(|b, a| {
        if is_line_safe(rules, &format!("{},{}", a, b)) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    unsafe_vec
}

fn part_two(input: &str) -> usize {
    let (page_ordering_rules, updates) = input.split_once("\n\n").unwrap();

    let rules = make_rules(page_ordering_rules);

    let unsafe_lines = updates.lines().filter(|line| !is_line_safe(&rules, line));

    let safe_lines = unsafe_lines.map(|line| make_line_safe(&rules, line));


    safe_lines
        .into_iter()
        .flat_map(|update| {
            update
                .iter()
                .take((update.len() / 2) + 1)
                .last()
                .unwrap()
                .parse::<usize>()
        })
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

    static INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 143);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 123);
    }
}
