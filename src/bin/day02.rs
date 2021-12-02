use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{} {0}")]
#[display(style = "snake_case")]
enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn read_input() -> Result<Vec<Direction>> {
    let input = File::open("input/day02.txt")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(|line| Ok(line?.parse()?)).collect()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let (x, y) = input.iter().fold((0, 0), |(x, y), dir| match dir {
        Direction::Up(n) => (x, y - n),
        Direction::Down(n) => (x, y + n),
        Direction::Forward(n) => (x + n, y),
    });

    println!("Part 1: {}", x * y);

    let (x, y, _) = input.iter().fold((0, 0, 0), |(x, y, aim), dir| match dir {
        Direction::Up(n) => (x, y, aim - n),
        Direction::Down(n) => (x, y, aim + n),
        Direction::Forward(n) => (x + n, y + aim * n, aim),
    });

    println!("Part 2: {}", x * y);

    Ok(())
}
