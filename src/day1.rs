use test_case::test_case;

#[test_case("inputs/input-01" => matches Ok(123))]
pub fn puzzle1(filename: &str) -> Result<u64, std::io::Error> {
    Ok(0)
}
