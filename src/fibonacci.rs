use num_bigint::BigUint;
use crate::util::{read_string, Error};
use crate::Result;
use std::ops::Add;

fn fib_rabbits(n: u32, k: u32) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }
    let mut f1: u64 = 1;
    let mut f2: u64 = 1;
    for _ in 3..=n {
        let next = f2 + k as u64 * f1;
        f1 = f2;
        f2 = next;
    }
    f2
}

fn mortal_fib_rabbits(n: u32, m: u32) -> BigUint {
    if n == 0 {
        return BigUint::from(0u32);
    }
    if n <= 2 {
        return BigUint::from(1u32);
    }

    // Keep track of rabbits by age (how many months they've been alive)
    // population[i] = number of rabbit pairs that are i+1 months old
    let mut population = vec![BigUint::from(0u32); m as usize];

    // Initially, we have 1 pair of newborns in month 1
    population[0] = BigUint::from(1u32); // 1-month-old rabbits

    for _month in 2..=n {
        let mut new_population = vec![BigUint::from(0u32); m as usize];

        // All mature rabbits (age >= 2 months) produce offspring
        let newborns: BigUint = population.iter().skip(1).fold(BigUint::from(0u32), |acc, x| acc.add(x));
        new_population[0] = newborns;

        // Age existing rabbits (they move to next age group if they don't die)
        for age in 1..(m as usize) {
            new_population[age] = population[age - 1].clone();
        }
        // Rabbits of age m die out (they don't get moved to new_population)

        population = new_population;
    }

    population.iter().fold(BigUint::from(0u32), |acc, x| acc.add(x))
}

pub fn execute_fib(input_file: &str) -> Result<()> {
    let content = read_string(input_file)?;
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(Error::Parse(
            "File must contain exactly two integers: n k".into(),
        ));
    }
    let n: u32 = parts[0].parse()?;
    let k: u32 = parts[1].parse()?;
    let result = fib_rabbits(n, k);
    println!("{}", result);
    Ok(())
}

pub fn execute_fibd(input_file: &str) -> Result<()> {
    let content = read_string(input_file)?;
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(Error::Parse(
            "File must contain exactly two integers: n k".into(),
        ));
    }
    let n: u32 = parts[0].parse()?;
    let m: u32 = parts[1].parse()?;
    let result = mortal_fib_rabbits(n, m);
    println!("{}", result);
    Ok(())
}