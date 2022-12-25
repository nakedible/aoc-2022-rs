use pathfinding::matrix;
use pathfinding::matrix::directions;
use pathfinding::matrix::Matrix;
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<bool>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .trim()
        .chars()
        .map(|x| match x {
            '<' => false,
            '>' => true,
            _ => panic!("invalid input"),
        })
        .collect();
    Ok(ret)
}

fn can_fit(board: &Matrix<bool>, shape: &Matrix<bool>, (pos_y, pos_x): (usize, usize)) -> bool {
    for ((y, x), b) in shape.items() {
        if !b {
            continue;
        }
        if let Some(false) = board.get((pos_y + y, pos_x + x)) {
            continue;
        }
        return false;
    }
    true
}

fn drop_piece(
    board: &mut Matrix<bool>,
    highest: usize,
    shape: &Matrix<bool>,
    jets: &mut impl Iterator<Item = bool>,
) -> usize {
    let mut landed = false;
    let mut pos = (highest + 3, 2);
    let mut spent = 0;
    let required_height = highest + 3 + shape.rows;
    while board.rows < required_height {
        board.extend(&[false; 7]).unwrap();
    }
    for right in jets {
        //println!("right: {}, pos: {:?}", right, pos);
        spent += 1;
        if let Some(newpos) =
            board.move_in_direction(pos, if right { directions::E } else { directions::W })
        {
            if can_fit(board, shape, newpos) {
                pos = newpos;
            }
        }
        if let Some(newpos) = board.move_in_direction(pos, directions::N) {
            if can_fit(board, shape, newpos) {
                pos = newpos;
                continue;
            }
        }
        //println!("landed, pos: {:?}", pos);
        landed = true;
        break;
    }
    if landed {
        for ((y, x), b) in shape.items() {
            if *b {
                board[(pos.0 + y, pos.1 + x)] = true;
            }
        }
    }
    spent
}

fn find_highest(board: &Matrix<bool>) -> usize {
    for y in (0..board.rows).rev() {
        for x in 0..board.columns {
            if board[(y, x)] {
                return y + 1;
            }
        }
    }
    0
}

#[allow(dead_code)]
fn draw_board(board: &Matrix<bool>) {
    for y in (0..board.rows).rev() {
        print!("|");
        for x in 0..board.columns {
            print!("{}", if board[(y, x)] { '#' } else { ' ' });
        }
        println!("|");
    }
    println!("+-------+");
}

#[test_case("inputs/example-17" => matches Ok(3068))]
#[test_case("inputs/input-17" => matches Ok(3173))]
pub fn puzzle1(filename: &str) -> Result<usize, std::io::Error> {
    let input = parse_input(filename)?;
    let shapes = [
        matrix![[true, true, true, true]],
        matrix![
            [false, true, false],
            [true, true, true],
            [false, true, false]
        ],
        matrix![
            [true, true, true],
            [false, false, true],
            [false, false, true]
        ],
        matrix![[true], [true], [true], [true]],
        matrix![[true, true], [true, true]],
    ];
    let mut board = Matrix::new_square(7, false);
    let mut highest: usize = 0;
    let mut curshape: usize = 0;
    let mut jets = input.into_iter().cycle();
    for _ in 0..2022 {
        drop_piece(&mut board, highest, &shapes[curshape], &mut jets);
        highest = find_highest(&board);
        curshape = (curshape + 1) % shapes.len();
    }
    //draw_board(&board);
    Ok(highest)
}

fn find_loop(highests: &[usize]) -> usize {
    for i in 10..10000 {
        let l = highests.len();
        if l < i * 2 {
            return 0;
        }
        if highests[(l - (i * 2))..(l - i)] == highests[(l - i)..] {
            return i;
        }
    }
    0
}

#[test_case("inputs/example-17" => matches Ok(1514285714288))]
#[test_case("inputs/input-17" => matches Ok(1570930232582))]
pub fn puzzle2(filename: &str) -> Result<usize, std::io::Error> {
    let input = parse_input(filename)?;
    let shapes = [
        matrix![[true, true, true, true]],
        matrix![
            [false, true, false],
            [true, true, true],
            [false, true, false]
        ],
        matrix![
            [true, true, true],
            [false, false, true],
            [false, false, true]
        ],
        matrix![[true], [true], [true], [true]],
        matrix![[true, true], [true, true]],
    ];
    let mut board = Matrix::new_square(7, false);
    let mut curshape: usize = 0;
    let mut jets = input.into_iter().cycle();
    let mut highests: Vec<usize> = Vec::new();
    let mut prev = 0;
    let target = 1000000000000usize;
    for _ in 0..10000u64 {
        let highest = find_highest(&board);
        highests.push(highest - prev);
        prev = highest;
        drop_piece(&mut board, highest, &shapes[curshape], &mut jets);
        curshape = (curshape + 1) % shapes.len();
    }
    let loopspan = find_loop(&highests);
    let loopheight: usize = highests[highests.len() - loopspan..].iter().sum();
    let loops = (target - 10000) / loopspan;
    for _ in 0..((target - 10000) % loopspan) {
        let highest = find_highest(&board);
        drop_piece(&mut board, highest, &shapes[curshape], &mut jets);
        curshape = (curshape + 1) % shapes.len();
    }
    //draw_board(&board);
    let ret = find_highest(&board) + (loopheight * loops);
    Ok(ret)
}
