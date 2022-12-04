use test_case::test_case;

type Range = (u64, u64);

fn parse_input(filename: &str) -> Result<Vec<(Range, Range)>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input
        .lines()
        .map(|x| {
            let (a, b) = x.split_once(',').unwrap();
            let (x1s, y1s) = a.split_once('-').unwrap();
            let (x2s, y2s) = b.split_once('-').unwrap();
            (
                (x1s.parse::<u64>().unwrap(), y1s.parse::<u64>().unwrap()),
                (x2s.parse::<u64>().unwrap(), y2s.parse::<u64>().unwrap()),
            )
        })
        .collect())
}

fn is_contained(a: Range, b: Range) -> bool {
    (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)
}

#[test_case("inputs/input-04" => matches Ok(509))]
pub fn puzzle1(filename: &str) -> Result<u64, std::io::Error> {
    let input = parse_input(filename)?;
    Ok(input.iter().filter(|(a, b)| is_contained(*a, *b)).count() as u64)
}

fn is_overlap(a: Range, b: Range) -> bool {
    a.1 < b.0 || a.0 > b.1
}

#[test_case("inputs/input-04" => matches Ok(870))]
pub fn puzzle2(filename: &str) -> Result<u64, std::io::Error> {
    let input = parse_input(filename)?;
    Ok(input.iter().filter(|(a, b)| !is_overlap(*a, *b)).count() as u64)
}
