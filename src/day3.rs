use std::collections::BTreeSet;
use test_case::test_case;

fn get_priority(item: char) -> u64 {
    match item {
        'a'..='z' => (item as u64) - ('a' as u64) + 1,
        'A'..='Z' => (item as u64) - ('A' as u64) + 27,
        _ => panic!("invalid input"),
    }
}

#[test_case("inputs/input-03" => matches Ok(8240))]
pub fn puzzle1(filename: &str) -> Result<u64, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let sets: Vec<(BTreeSet<char>, BTreeSet<char>)> = input
        .lines()
        .map(|x| {
            let half = x.len() / 2;
            (x[..half].chars().collect(), x[half..].chars().collect())
        })
        .collect();
    let result = sets
        .iter()
        .map(|(a, b)| a.intersection(b).next().unwrap().to_owned())
        .map(get_priority)
        .sum();
    Ok(result)
}

#[test_case("inputs/input-03" => matches Ok(2587))]
pub fn puzzle2(filename: &str) -> Result<u64, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let sets: Vec<BTreeSet<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let result = sets
        .chunks(3)
        .map(|x| {
            x[0].intersection(&x[1])
                .copied()
                .collect::<BTreeSet<char>>()
                .intersection(&x[2])
                .next()
                .unwrap()
                .to_owned()
        })
        .map(get_priority)
        .sum();
    Ok(result)
}
