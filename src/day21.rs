use test_case::test_case;

fn parse_input(filename: &str) -> Result<String, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input)
}

#[test_case("inputs/input-17" => matches Ok(4300))]
pub fn puzzle1(filename: &str) -> Result<usize, std::io::Error> {
    let input = parse_input(filename)?;
    Ok(input.len())
}
