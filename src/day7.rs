use std::collections::BTreeMap;
use std::path::PathBuf;
use test_case::test_case;

pub fn parse_input(filename: &str) -> Result<BTreeMap<String, u64>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let mut curpath = PathBuf::from("/");
    let mut sizes: BTreeMap<String, u64> = BTreeMap::new();
    for line in input.lines() {
        if let Some(d) = line.strip_prefix("$ cd ") {
            if d == ".." {
                // nnngh, absolute is nightly
                curpath.pop();
            } else {
                curpath.push(d);
            }
        } else if line.chars().next().unwrap().is_ascii_digit() {
            let (size, _fn) = line.split_once(' ').unwrap();
            let size = size.parse::<u64>().unwrap();
            let mut tmppath = PathBuf::new();
            for part in curpath.iter() {
                tmppath.push(part);
                *sizes
                    .entry(tmppath.to_string_lossy().to_string())
                    .or_insert(0) += size;
            }
        }
    }
    Ok(sizes)
}

#[test_case("inputs/input-07" => matches Ok(1447046))]
pub fn puzzle1(filename: &str) -> Result<u64, std::io::Error> {
    let sizes = parse_input(filename)?;
    Ok(sizes.values().filter(|&&x| x <= 100000).sum())
}

#[test_case("inputs/input-07" => matches Ok(578710))]
pub fn puzzle2(filename: &str) -> Result<u64, std::io::Error> {
    let sizes = parse_input(filename)?;
    let total_size = *sizes.get("/").unwrap();
    let needed = 30000000 - (70000000 - total_size);
    Ok(*sizes.values().filter(|&&x| x >= needed).min().unwrap())
}
