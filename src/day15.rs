use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<((i64, i64), (i64, i64))>, std::io::Error> {
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

fn build_spans(input: &Vec<((i64, i64), (i64, i64))>, desired_line: i64) -> Vec<(i64, bool)> {
    let mut spans: Vec<(i64, bool)> = input
        .iter()
        .flat_map(|((sx, sy), (dx, dy))| {
            let dist = (sx - dx).abs() + (sy - dy).abs();
            let ydist = (desired_line - sy).abs();
            let xdist = dist - ydist;
            //dbg!(desired_line, *sx, *sy, *dx, *dy, dist, ydist, xdist, sx - xdist, sx + xdist);
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

// fn draw_map(input: &Vec<((i64, i64), (i64, i64))>) {
//     let mut map = [['.'; 26]; 26];
//     for y in 0..=25 {
//         let spans = build_spans(input, y);
//         let mut nest = 0;
//         let mut last_x = 0;
//         for (x, is_end) in spans {
//             if nest > 0 {
//                 for cur_x in last_x..=x {
//                     if cur_x >= 0 && cur_x <= 25 {
//                         map[y as usize][cur_x as usize] = '#';
//                     }
//                 }
//             }
//             if is_end {
//                 nest -= 1;
//             } else {
//                 nest += 1;
//             }
//             last_x = x;
//         }
//     }
//     for ((sx, sy), (bx, by)) in input {
//         if *sx >= 0 && *sx <= 25 {
//             map[*sy as usize][*sx as usize] = 'S';
//         }
//         if *bx >= 0 && *bx <= 25 {
//             map[*by as usize][*bx as usize] = 'B';
//         }
//     }
//     map.iter()
//         .for_each(|l| println!("{}", l.iter().collect::<String>()));
// }

#[test_case("inputs/example-15", 20 => matches Ok(56000011))]
#[test_case("inputs/input-15", 4000000 => matches Ok(0))]
pub fn puzzle2(filename: &str, max_coord: i64) -> Result<i64, std::io::Error> {
    let input = parse_input(filename)?;
    //draw_map(&input);
    for y in 0..=max_coord {
        if let (Some((x, y)), _) = {
            let spans = build_spans(&input, y);
            //dbg!(&spans);
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
            //dbg!(x, y);
            return Ok(x * 4000000 + y);
        }
    }
    Ok(0)
}
