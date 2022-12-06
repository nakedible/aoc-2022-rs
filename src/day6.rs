use test_case::test_case;

fn has_dups(window: &[char]) -> bool {
    for i in 1..window.len() {
        if window[i..].contains(&window[i - 1]) {
            return true;
        }
    }
    false
}

#[test_case("inputs/input-06", 4 => matches Ok(1850))]
#[test_case("inputs/input-06", 14 => matches Ok(2823))]
pub fn puzzle12(filename: &str, window_size: usize) -> Result<usize, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let chars: Vec<char> = input.chars().collect();
    let (i, _) = chars
        .windows(window_size)
        .enumerate()
        .find(|x| !has_dups(x.1))
        .unwrap();
    Ok(i + window_size)
}
