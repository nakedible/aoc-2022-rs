use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<(char, char)>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input
        .lines()
        .map(|x| {
            let mut s = x.split_whitespace();
            (
                s.next().unwrap().chars().next().unwrap(),
                s.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect())
}

#[allow(clippy::identity_op)]
fn calc_score(hand: (char, char)) -> u64 {
    match hand {
        ('A', 'X') => 1 + 3,
        ('B', 'X') => 1 + 0,
        ('C', 'X') => 1 + 6,
        ('A', 'Y') => 2 + 6,
        ('B', 'Y') => 2 + 3,
        ('C', 'Y') => 2 + 0,
        ('A', 'Z') => 3 + 0,
        ('B', 'Z') => 3 + 6,
        ('C', 'Z') => 3 + 3,
        _ => panic!("invalid input"),
    }
}

#[test_case("inputs/input-02" => matches Ok(13446))]
pub fn puzzle1(filename: &str) -> Result<u64, std::io::Error> {
    let input = parse_input(filename)?;
    Ok(input.iter().map(|&x| calc_score(x)).sum())
}

fn calc_desired(hand: (char, char)) -> (char, char) {
    match hand {
        ('A', 'X') => ('A', 'Z'),
        ('B', 'X') => ('B', 'X'),
        ('C', 'X') => ('C', 'Y'),
        ('A', 'Y') => ('A', 'X'),
        ('B', 'Y') => ('B', 'Y'),
        ('C', 'Y') => ('C', 'Z'),
        ('A', 'Z') => ('A', 'Y'),
        ('B', 'Z') => ('B', 'Z'),
        ('C', 'Z') => ('C', 'X'),
        _ => panic!("invalid input"),
    }
}

#[test_case("inputs/input-02" => matches Ok(13509))]
pub fn puzzle2(filename: &str) -> Result<u64, std::io::Error> {
    let input = parse_input(filename)?;
    Ok(input.iter().map(|&x| calc_score(calc_desired(x))).sum())
}
