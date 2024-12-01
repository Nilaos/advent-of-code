use std::iter::zip;

use aoc_driver::*;
use nom::{
    character::complete::{digit1, one_of},
    combinator::recognize,
    multi::many0,
    sequence::{terminated, tuple},
    IResult, Parser,
};

fn decimal(input: &str) -> IResult<&str, &str> {
    // println!("Matching at {:?}", input.get(0..4));
    terminated(digit1, many0(one_of(" \n"))).parse(input)
}

fn get_column_nums(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((decimal, decimal)).parse(input)
}

fn solution1(i: &str) -> String {
    let mut v0: Vec<i32> = vec![];
    let mut v1: Vec<i32> = vec![];
    let zipped = many0(get_column_nums).parse(i).expect("Doesn't work!").1;
    zipped.iter().for_each(|e| {
        v0.push(e.0.parse().unwrap());
        v1.push(e.1.parse().unwrap())
    });

    v1.sort();
    v0.sort();
    let res: i32 = zip(v0, v1).map(|e| (e.0 - e.1).abs()).sum();
    println!("{res}");
    res.to_string()
}

fn solution2(i: &str) -> String {
    let mut v0: Vec<i32> = vec![];
    let mut v1: Vec<i32> = vec![];
    let zipped = many0(get_column_nums).parse(i).expect("Doesn't work!").1;
    zipped.iter().for_each(|e| {
        v0.push(e.0.parse().unwrap());
        v1.push(e.1.parse().unwrap())
    });

    // Count number of times each elem in v0 appears in v1, sum those
    let res: i64 = v0
        .iter()
        .map(|&e| {
            let count = i64::try_from(v1.iter().filter(|&&e2| e == e2).count()).unwrap();
            // println!(
            //     "{} appears {} times... adding {}",
            //     e,
            //     count,
            //     i64::from(e) * count
            // );
            i64::from(e) * count
        })
        .sum();
    println!("{res}");
    res.to_string()
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2024:1:2, solution2).unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs::{self};

    use crate::{get_column_nums, solution1, solution2};

    #[test]
    fn line() {
        assert_eq!(
            get_column_nums("80421   40193\n"),
            Ok(("", ("80421", "40193")))
        )
    }

    #[test]
    fn answer() {
        let f = fs::read_to_string("inputs/2024/1.txt").unwrap();
        println!("{}", solution1(f.as_str()));
    }

    #[test]
    fn ans2() {
        let f = fs::read_to_string("inputs/2024/1.txt").unwrap();
        println!("{}", solution2(f.as_str()));
    }
}
