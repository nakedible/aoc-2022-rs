use test_case::test_case;

type Reading = ((i64, i64), (i64, i64));

fn parse_input(filename: &str) -> Result<Vec<Reading>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .lines()
        .map(|x| {
            let mut nums = x
                .split(|c: char| !c.is_numeric() && c != '-')
                .filter(|v| !v.is_empty())
                .map(|y| y.parse().unwrap());
            (
                (nums.next().unwrap(), nums.next().unwrap()),
                (nums.next().unwrap(), nums.next().unwrap()),
            )
        })
        .collect();
    Ok(ret)
}

fn build_spans(input: &[Reading], desired_line: i64) -> Vec<(i64, bool)> {
    let mut spans: Vec<(i64, bool)> = input
        .iter()
        .flat_map(|((sx, sy), (dx, dy))| {
            let dist = (sx - dx).abs() + (sy - dy).abs();
            let ydist = (desired_line - sy).abs();
            let xdist = dist - ydist;
            if xdist >= 0 {
                vec![(sx - xdist, false), (sx + xdist, true)]
            } else {
                vec![]
            }
        })
        .collect();
    spans.sort();
    spans
}

#[test_case("inputs/example-15", 10 => matches Ok(26))]
#[test_case("inputs/input-15", 2000000 => matches Ok(5147333))]
pub fn puzzle1(filename: &str, desired_line: i64) -> Result<i64, std::io::Error> {
    let input = parse_input(filename)?;
    let spans = build_spans(&input, desired_line);
    let (count, _, _) = spans
        .iter()
        .fold((0, 0, 0), |(count, start, nest), (x, is_end)| {
            if *is_end && nest == 1 {
                (count + (*x - start), 0, 0)
            } else if *is_end {
                (count, start, nest - 1)
            } else if nest == 0 {
                (count, *x, 1)
            } else {
                (count, start, nest + 1)
            }
        });
    Ok(count)
}

#[test_case("inputs/example-15", 20 => matches Ok(56000011))]
#[test_case("inputs/input-15", 4000000 => matches Ok(13734006908372))]
pub fn puzzle2(filename: &str, max_coord: i64) -> Result<i64, std::io::Error> {
    let input = parse_input(filename)?;
    for y in 0..=max_coord {
        if let (Some((x, y)), _) = {
            let spans = build_spans(&input, y);
            spans.iter().fold((None, 0), |(found, nest), (x, is_end)| {
                if *is_end && nest == 1 && (*x + 1) >= 0 && (*x + 1) <= max_coord {
                    (Some((*x + 1, y)), 0)
                } else if *is_end {
                    (found, nest - 1)
                } else if nest == 0 && (*x - 1) >= 0 && (*x - 1) <= max_coord {
                    (Some((*x - 1, y)), 1)
                } else {
                    (found, nest + 1)
                }
            })
        } {
            return Ok(x * 4000000 + y);
        }
    }
    Ok(0)
}
