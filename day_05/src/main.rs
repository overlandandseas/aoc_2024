use std::collections::{HashMap, HashSet};

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

fn part_one(input: &str) -> usize {
    let mut rules: HashMap<&str, PageOrdering> = HashMap::new();
    let (page_ordering_rules, updates) = input.split_once("\n\n").unwrap();

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

    let mut safe_updates = vec![];

    'outer: for update in updates.lines() {
        let mut page_numbers = update.split(",");

        while let Some(page_number) = page_numbers.next() {
            if let Some(page) = rules.get(page_number) {
                if !page.safe(page_numbers.clone(), page_number) {
                    continue 'outer;
                }
            }
        }

        safe_updates.push(update);
    }

    safe_updates
        .into_iter()
        .map(|update| {
            let length = update.split(",").count();

            update
                .split(",")
                .take((length / 2) + 1)
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input");

    println!("Part One: {}", part_one(input));
}

#[cfg(test)]

mod tests {
    use crate::part_one;

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
}
