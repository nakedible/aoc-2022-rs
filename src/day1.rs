use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<Vec<u64>>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let lines: Vec<&str> = input.lines().collect();
    let elves: Vec<Vec<u64>> = lines
        .split(|&x| x.is_empty())
        .map(|x| {
            x.iter()
                .map(|y| y.parse::<u64>().expect("invalid number"))
                .collect()
        })
        .collect();
    Ok(elves)
}

#[test_case("inputs/input-01" => matches Ok(68442))]
pub fn puzzle1(filename: &str) -> Result<u64, std::io::Error> {
    let elves = parse_input(filename)?;
    Ok(elves.iter().map(|elf| elf.iter().sum()).max().unwrap())
}

#[test_case("inputs/input-01" => matches Ok(204837))]
pub fn puzzle2(filename: &str) -> Result<u64, std::io::Error> {
    let elves = parse_input(filename)?;
    let mut sums: Vec<u64> = elves.iter().map(|elf| elf.iter().sum()).collect();
    sums.sort();
    Ok(sums.iter().rev().take(3).sum())
}
