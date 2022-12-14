use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<Vec<(usize, usize)>>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|x| {
                    let (x, y) = x.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect())
}

fn draw_grid(shapes: Vec<Vec<(usize, usize)>>, floor: bool) -> [[u8; 1000]; 200] {
    let mut grid = [[0_u8; 1000]; 200];
    if floor {
        let maxy = shapes
            .iter()
            .map(|x| x.iter().map(|y| y.1).max().unwrap())
            .max()
            .unwrap();
        for x in 0..1000 {
            grid[maxy + 2][x] = 1;
        }
    }
    for shape in shapes {
        for v in shape.windows(2) {
            match v {
                [(sx, sy), (dx, dy)] if sx == dx => {
                    let range = if dy >= sy { *sy..=*dy } else { *dy..=*sy };
                    for y in range {
                        grid[y][*sx] = 1;
                    }
                }
                [(sx, sy), (dx, dy)] if sy == dy => {
                    let range = if dx >= sx { *sx..=*dx } else { *dx..=*sx };
                    for x in range {
                        grid[*sy][x] = 1;
                    }
                }
                _ => panic!("invalid input"),
            }
        }
    }
    grid
}

fn drop_sand(grid: &mut [[u8; 1000]; 200]) -> bool {
    let mut cx = 500;
    let mut cy = 0;
    loop {
        if grid[cy][cx] != 0 || cy >= 199 {
            return false;
        } else if grid[cy + 1][cx] == 0 {
            cy += 1;
        } else if grid[cy + 1][cx - 1] == 0 {
            cy += 1;
            cx -= 1;
        } else if grid[cy + 1][cx + 1] == 0 {
            cy += 1;
            cx += 1;
        } else {
            grid[cy][cx] = 2;
            return true;
        }
    }
}

#[test_case("inputs/example-14" => matches Ok(24))]
#[test_case("inputs/input-14" => matches Ok(1061))]
pub fn puzzle1(filename: &str) -> Result<usize, std::io::Error> {
    let input = parse_input(filename)?;
    let mut count = 0;
    let mut grid = draw_grid(input, false);
    while drop_sand(&mut grid) {
        count += 1;
    }
    Ok(count)
}

#[test_case("inputs/input-14" => matches Ok(25055))]
pub fn puzzle2(filename: &str) -> Result<usize, std::io::Error> {
    let input = parse_input(filename)?;
    let mut count = 0;
    let mut grid = draw_grid(input, true);
    while drop_sand(&mut grid) {
        count += 1;
    }
    Ok(count)
}
