use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap},
    iter::zip,
};

use aoc_driver::*;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of},
    combinator::recognize,
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

fn _decimal(input: &str) -> IResult<&str, &str> {
    // println!("Matching at {:?}", input.get(0..4));
    terminated(digit1, many0(one_of(" \n"))).parse(input)
}

// #[derive(Debug, Eq, Ord)]
// struct PageOrder {
//     page_num: String,
//     priority: i32,
// }

// impl PartialOrd for PageOrder {
//     fn partial_cmp(&self, other: &PageOrder) -> Option<Ordering> {
//         self.priority.partial_cmp(&other.priority)
//     }
// }

// impl PartialEq for PageOrder {
//     fn eq(&self, other: &PageOrder) -> bool {
//         self.priority.eq(&other.priority)
//     }
// }

// impl From<(&&str, &i32)> for PageOrder {
//     fn from(value: (&&str, &i32)) -> Self {
//         PageOrder {
//             page_num: value.0.to_string(),
//             priority: value.1.clone(),
//         }
//     }
// }

#[derive(Clone)]
pub struct PageRules {
    pub before: Vec<usize>,
    pub after: Vec<usize>,
}

fn get_ordering(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(terminated(
        tuple((digit1, preceded(tag("|"), digit1))),
        newline,
    ))
    .parse(input)
}

fn get_pages(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    many1(preceded(newline, separated_list1(tag(","), digit1))).parse(input)
}

fn solution1(i: &str) -> String {
    let (partial, orders) = get_ordering(i).unwrap();
    let (_none, pages) = get_pages(partial).unwrap();
    let mut priorities = HashMap::new();
    orders.iter().for_each(|elem| {
        let (first, second) = elem.clone();

        let f = priorities.entry(first).or_insert(PageRules {
            before: vec![],
            after: vec![],
        });
        f.after.push(second.parse().unwrap());

        let s = priorities.entry(second).or_insert(PageRules {
            before: vec![],
            after: vec![],
        });
        s.before.push(first.parse().unwrap());
    });

    // let mut priority_list: Vec<PageOrder> = priorities.iter().map(|e| e.into()).collect();
    // priority_list.sort();

    // println!("{:?}", priority_list);

    let result: i32 = pages
        .iter()
        .map(|update| {
            // Sum middle page numbers of valid updates
            if update.is_sorted_by(|&a, &b| {
                // priorities.get(a).unwrap() < priorities.get(b).unwrap()
                !(priorities
                    .get(a)
                    .unwrap()
                    .before
                    .contains(&b.parse::<usize>().unwrap()))
                    && !(priorities
                        .get(b)
                        .unwrap()
                        .after
                        .contains(&a.parse().unwrap()))
            }) {
                update
                    .get(update.len() / 2)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
            } else {
                0
            }
        })
        .sum();

    result.to_string()
}

fn solution2(i: &str) -> String {
    let (partial, orders) = get_ordering(i).unwrap();
    let (_none, mut pages) = get_pages(partial).unwrap();
    let mut priorities = HashMap::new();
    orders.iter().for_each(|elem| {
        let (first, second) = elem.clone();

        let f = priorities.entry(first).or_insert(PageRules {
            before: vec![],
            after: vec![],
        });
        f.after.push(second.parse().unwrap());

        let s = priorities.entry(second).or_insert(PageRules {
            before: vec![],
            after: vec![],
        });
        s.before.push(first.parse().unwrap());
    });

    // let mut priority_list: Vec<PageOrder> = priorities.iter().map(|e| e.into()).collect();
    // priority_list.sort();

    // println!("{:?}", priority_list);

    let result: i32 = pages
        .iter_mut()
        .filter(|update| {
            // Sum middle page numbers of valid updates
            !update.is_sorted_by(|&a, &b| {
                // priorities.get(a).unwrap() < priorities.get(b).unwrap()
                !(priorities
                    .get(a)
                    .unwrap()
                    .before
                    .contains(&b.parse::<usize>().unwrap()))
                    && !(priorities
                        .get(b)
                        .unwrap()
                        .after
                        .contains(&a.parse().unwrap()))
            })
        })
        .map(|update| {
            update.sort_by(|&p1, &p2| {
                if priorities.get(p1).unwrap().after.contains(&p2.parse().unwrap()) || priorities.get(p2).unwrap().before.contains(&p1.parse().unwrap()) {
                    Ordering::Less
                } else if priorities.get(p1).unwrap().before.contains(&p2.parse().unwrap()) || priorities.get(p2).unwrap().after.contains(&p1.parse().unwrap()) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update
                    .get(update.len() / 2)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
        }).sum();
    

    result.to_string()
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2024:5:2, solution2).unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs::{self};

    use crate::{solution1, solution2};

    #[test]
    fn line() {
        let sample = "47|53
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
        assert_eq!(solution2(sample), "143");
    }

    #[test]
    fn answer() {
        let f = fs::read_to_string("day5/inputs/2024/5.txt").unwrap();
        eprintln!("{}", solution1(f.as_str()));
    }

    #[test]
    fn ans2() {
        let f = fs::read_to_string("inputs/2024/1.txt").unwrap();
        println!("{}", solution2(f.as_str()));
    }
}
