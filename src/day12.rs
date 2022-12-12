use pathfinding::directed::astar::astar;
use test_case::test_case;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

fn parse_input<const XSIZE: usize, const YSIZE: usize>(
    filename: &str,
) -> Result<([[u8; XSIZE]; YSIZE], Coords, Coords), std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let mut ret = [[0; XSIZE]; YSIZE];
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            ret[y][x] = match char {
                'S' => {
                    start = Some(Coords { x, y });
                    0
                }
                'E' => {
                    end = Some(Coords { x, y });
                    26
                }
                'a'..='z' => (char as u8) - (b'a') + 1,
                _ => panic!("invalid input: {}", char),
            };
        }
    }
    Ok((ret, start.unwrap(), end.unwrap()))
}

fn find_path_len<const XSIZE: usize, const YSIZE: usize>(
    input: [[u8; XSIZE]; YSIZE],
    start: Coords,
    end: Coords,
) -> Option<usize> {
    let ret = astar(
        &start,
        |p| {
            let mut ret = Vec::new();
            if p.x == 255 && p.y == 255 {
                for y in 0..YSIZE {
                    for x in 0..XSIZE {
                        if input[y][x] == 1 {
                            ret.push((Coords { x, y}, 1));
                        }
                    }
                }
                return ret;
            }
            for (a, b) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
                let dx = p.x as i64 + a;
                let dy = p.y as i64 + b;
                if dx < 0 || dx >= XSIZE as i64 || dy < 0 || dy >= YSIZE as i64 {
                    continue;
                }
                if (input[p.y][p.x] + 1) < input[dy as usize][dx as usize] {
                    continue;
                }
                ret.push((
                    Coords {
                        x: dx as usize,
                        y: dy as usize,
                    },
                    1,
                ));
            }
            ret
        },
        |p| (p.x as i64 - end.x as i64).abs() + (p.y as i64 - end.y as i64).abs(),
        |p| *p == end,
    );
    if let Some((steps, _count)) = ret {
        Some(steps.len() - 1)
    } else {
        None
    }
}

#[test_case("inputs/input-12" => matches Ok(528))]
pub fn puzzle1(filename: &str) -> Result<usize, std::io::Error> {
    const XSIZE: usize = 181;
    const YSIZE: usize = 41;
    let (input, start, end) = parse_input::<XSIZE, YSIZE>(filename)?;
    Ok(find_path_len(input, start, end).unwrap())
}

#[test_case("inputs/input-12" => matches Ok(522))]
pub fn puzzle2(filename: &str) -> Result<usize, std::io::Error> {
    const XSIZE: usize = 181;
    const YSIZE: usize = 41;
    let (input, _start, end) = parse_input::<XSIZE, YSIZE>(filename)?;
    Ok(find_path_len(input, Coords { x: 255, y: 255 }, end).unwrap() - 1)
}
