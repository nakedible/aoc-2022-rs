use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<(usize, usize, usize)>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input
        .lines()
        .map(|l| {
            let (x, rest) = l.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect())
}

fn calc_edges(grid: &[[[u8; 32]; 32]; 32], pos: (usize, usize, usize)) -> u8 {
    let (x, y, z) = pos;
    grid[z][y][x]
        * (6 - grid[z][y][x - 1]
            - grid[z][y][x + 1]
            - grid[z][y - 1][x]
            - grid[z][y + 1][x]
            - grid[z - 1][y][x]
            - grid[z + 1][y][x])
}

#[test_case("inputs/example-18" => matches Ok(64))]
#[test_case("inputs/input-18" => matches Ok(4300))]
pub fn puzzle1(filename: &str) -> Result<i64, std::io::Error> {
    let input = parse_input(filename)?;
    let mut grid = [[[0u8; 32]; 32]; 32];
    input
        .iter()
        .for_each(|&(x, y, z)| grid[z + 1][y + 1][x + 1] = 1);
    let mut count = 0;
    for z in 1..31 {
        for y in 1..31 {
            for x in 1..31 {
                count += calc_edges(&grid, (x, y, z)) as i64;
            }
        }
    }
    Ok(count)
}

fn flood_fill(grid: &mut [[[u8; 32]; 32]; 32]) {
    let mut todo = Vec::new();
    todo.push((1, 1, 1));
    while let Some((x, y, z)) = todo.pop() {
        if (1..32).contains(&z) && (1..32).contains(&y) && (1..32).contains(&x) && grid[z][y][x] == 0 {
            grid[z][y][x] = 2;
            todo.push((x + 1, y, z));
            todo.push((x - 1, y, z));
            todo.push((x, y + 1, z));
            todo.push((x, y - 1, z));
            todo.push((x, y, z + 1));
            todo.push((x, y, z - 1));
        }
    }
}

fn calc_filled(grid: &[[[u8; 32]; 32]; 32], pos: (usize, usize, usize)) -> u8 {
    let (x, y, z) = pos;
    if grid[z][y][x] == 1 {
        (grid[z][y][x - 1] == 2) as u8
            + (grid[z][y][x + 1] == 2) as u8
            + (grid[z][y - 1][x] == 2) as u8
            + (grid[z][y + 1][x] == 2) as u8
            + (grid[z - 1][y][x] == 2) as u8
            + (grid[z + 1][y][x] == 2) as u8
    } else {
        0
    }
}

#[test_case("inputs/example-18" => matches Ok(58))]
#[test_case("inputs/input-18" => matches Ok(2490))]
pub fn puzzle2(filename: &str) -> Result<i64, std::io::Error> {
    let input = parse_input(filename)?;
    let mut grid = [[[0u8; 32]; 32]; 32];
    input
        .iter()
        .for_each(|&(x, y, z)| grid[z + 2][y + 2][x + 2] = 1);
    flood_fill(&mut grid);
    let mut count = 0;
    for z in 1..31 {
        for y in 1..31 {
            for x in 1..31 {
                count += calc_filled(&grid, (x, y, z)) as i64;
            }
        }
    }
    Ok(count)
}
