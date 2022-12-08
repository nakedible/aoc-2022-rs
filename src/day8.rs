use test_case::test_case;

pub fn parse_input<const L: usize>(filename: &str) -> Result<[[u8; L]; L], std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; L]>>()
        .try_into()
        .unwrap();
    Ok(ret)
}

fn is_visible<const L: usize>(my_x: usize, my_y: usize, grid: &[[u8; L]; L]) -> bool {
    let height = grid[my_y][my_x];
    (0..my_x)
        .map(|x| grid[my_y][x] < height)
        .all(std::convert::identity)
        || ((my_x + 1)..L)
            .map(|x| grid[my_y][x] < height)
            .all(std::convert::identity)
        || (0..my_y)
            .map(|y| grid[y][my_x] < height)
            .all(std::convert::identity)
        || ((my_y + 1)..L)
            .map(|y| grid[y][my_x] < height)
            .all(std::convert::identity)
}

#[test_case("inputs/input-08" => matches Ok(1676))]
pub fn puzzle1(filename: &str) -> Result<usize, std::io::Error> {
    const SIZ: usize = 99;
    let input = parse_input::<SIZ>(filename)?;
    let mut count: usize = SIZ + SIZ + SIZ - 2 + SIZ - 2; // edges
    for x in 1..(SIZ - 1) {
        for y in 1..(SIZ - 1) {
            if is_visible(x, y, &input) {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn calc_score<const L: usize>(my_x: usize, my_y: usize, grid: &[[u8; L]; L]) -> u64 {
    let height = grid[my_y][my_x];
    let left = my_x
        - (0..my_x)
            .rev()
            .find(|&x| grid[my_y][x] >= height)
            .unwrap_or(0);
    let right = ((my_x + 1)..L)
        .find(|&x| grid[my_y][x] >= height)
        .unwrap_or(L - 1)
        - my_x;
    let up = my_y
        - (0..my_y)
            .rev()
            .find(|&y| grid[y][my_x] >= height)
            .unwrap_or(0);
    let down = ((my_y + 1)..L)
        .find(|&y| grid[y][my_x] >= height)
        .unwrap_or(L - 1)
        - my_y;
    //dbg!(left, right, up, down);
    (left * right * up * down) as u64
}

#[test_case("inputs/input-08" => matches Ok(313200))]
pub fn puzzle2(filename: &str) -> Result<u64, std::io::Error> {
    const SIZ: usize = 99;
    let input = parse_input::<SIZ>(filename)?;
    let ret = (1..(SIZ - 1))
        .map(|x| {
            (1..(SIZ - 1))
                .map(|y| calc_score(x, y, &input))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    Ok(ret)
}
