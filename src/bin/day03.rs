use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};

const BIT_STRING_SIZE: u8 = 12;

fn read_input() -> Result<Vec<u16>> {
    let input = File::open("input/day03.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| {
            // Reverse the line so we get the right endianness
            let line = line?.chars().rev().collect::<String>();
            Ok(u16::from_str_radix(&line, 2)?)
        })
        .collect()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let mut gamma: u16 = 0;
    let mut epsilon: u16 = 0;
    for n in 0..BIT_STRING_SIZE {
        let ones = input
            .iter()
            .map(|x| x & (1 << n) != 0)
            .filter(|x| *x)
            .count();

        let p = BIT_STRING_SIZE - n - 1;

        if ones >= input.len() - ones {
            gamma = gamma | (1 << p);
        } else {
            epsilon = epsilon | (1 << p);
        }
    }

    println!("Part 1 {}", gamma as u64 * epsilon as u64);

    Ok(())
}
