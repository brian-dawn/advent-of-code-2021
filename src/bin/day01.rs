use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use itertools::Itertools;

fn read_input() -> Result<Vec<i32>> {
    let input = File::open("input/day1.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| Ok(line?.parse::<i32>()?))
        .collect()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let part1 = input.iter().tuple_windows().filter(|(a, b)| a < b).count();

    println!("Part 1: {}", part1);

    let v = input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect::<Vec<i32>>();

    let part2 = v
        .iter()
        .zip(v.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count();

    println!("Part 2: {}", part2);

    Ok(())
}
