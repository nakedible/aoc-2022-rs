use pathfinding::matrix;
use pathfinding::matrix::{directions, Matrix};
use test_case::test_case;

#[derive(Debug)]
enum Turn {
    None,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    count: usize,
    turn: Turn,
}

fn parse_input(filename: &str) -> Result<(Matrix<char>, Vec<Move>), std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let (board, path) = input.split_once("\n\n").unwrap();
    let cols = board.lines().map(str::len).max().unwrap();
    let board = Matrix::from_rows(
        board
            .lines()
            .map(|l| l.chars().chain(std::iter::repeat(' ').take(cols - l.len()))),
    )
    .unwrap();
    let path = path
        .trim()
        .split_inclusive(&['R', 'L'])
        .map(|x| match &x[x.len() - 1..] {
            "R" => Move {
                count: x[..x.len() - 1].parse().unwrap(),
                turn: Turn::Right,
            },
            "L" => Move {
                count: x[..x.len() - 1].parse().unwrap(),
                turn: Turn::Left,
            },
            _ => Move {
                count: x.parse().unwrap(),
                turn: Turn::None,
            },
        })
        .collect();
    Ok((board, path))
}

fn move_wrap(board: &Matrix<char>, pos: (usize, usize), facing: (isize, isize)) -> (usize, usize) {
    (
        (pos.0 as isize + facing.0).rem_euclid(board.rows as isize) as usize,
        (pos.1 as isize + facing.1).rem_euclid(board.columns as isize) as usize,
    )
}

fn do_turn(facing: (isize, isize), turn: Turn) -> (isize, isize) {
    match (facing, turn) {
        (directions::N, Turn::Right) => directions::E,
        (directions::E, Turn::Right) => directions::S,
        (directions::S, Turn::Right) => directions::W,
        (directions::W, Turn::Right) => directions::N,
        (directions::N, Turn::Left) => directions::W,
        (directions::E, Turn::Left) => directions::N,
        (directions::S, Turn::Left) => directions::E,
        (directions::W, Turn::Left) => directions::S,
        (_, Turn::None) => facing,
        _ => panic!("invalid turn"),
    }
}

fn do_move(
    board: &Matrix<char>,
    pos: (usize, usize),
    facing: (isize, isize),
    count: usize,
) -> (usize, usize) {
    let mut count = count;
    let mut last_pos = pos;
    let mut cur_pos = pos;
    while count > 0 {
        cur_pos = move_wrap(board, cur_pos, facing);
        match board[cur_pos] {
            ' ' => {
                continue;
            }
            '.' => {
                count -= 1;
                last_pos = cur_pos;
            }
            '#' => {
                break;
            }
            _ => panic!("invalid value in board"),
        }
    }
    last_pos
}

fn get_facing_val(facing: (isize, isize)) -> usize {
    match facing {
        directions::E => 0,
        directions::S => 1,
        directions::W => 2,
        directions::N => 3,
        _ => panic!("invalid facing"),
    }
}

#[test_case("inputs/example-22" => matches Ok(6032))]
#[test_case("inputs/input-22" => matches Ok(191010))]
pub fn puzzle1(filename: &str) -> Result<usize, std::io::Error> {
    let (board, path) = parse_input(filename)?;
    let mut facing = directions::E;
    let mut pos = (0, 0);
    pos = do_move(&board, pos, facing, 1);
    for step in path {
        pos = do_move(&board, pos, facing, step.count);
        facing = do_turn(facing, step.turn);
    }
    let password = ((pos.0 + 1) * 1000) + ((pos.1 + 1) * 4) + get_facing_val(facing);
    Ok(password)
}

#[derive(Debug)]

enum Face {
    Front,
    Left,
    Right,
    Top,
    Bottom,
    Back,
}

fn minimize(board: &Matrix<char>, size: usize) -> Matrix<u8> {
    let mut ret = Matrix::new(board.rows / size, board.columns / size, 0);
    for y in 0..ret.rows {
        for x in 0..ret.columns {
            ret[(y, x)] = (board[(y * size, x * size)] != ' ') as u8;
        }
    }
    ret
}

fn find_hexamino(board: &Matrix<char>, hexominoes: &[Matrix<u8>]) -> (bool, usize, usize, usize) {
    let (size, sideways) = match (board.rows, board.columns) {
        (r, c) if r / 4 == c / 3 => (r / 4, false),
        (r, c) if r / 3 == c / 4 => (r / 3, true),
        (r, c) if r / 5 == c / 2 => (r / 5, false),
        (r, c) if r / 2 == c / 5 => (r / 2, true),
        _ => panic!("invalid board size"),
    };
    let small = minimize(board, size);
    let small = if sideways { small.rotated_cw(1) } else { small };
    let all_smalls = vec![
        small.clone(),
        small.flipped_lr(),
        small.flipped_ud(),
        small.flipped_lr().flipped_ud(),
    ];
    let shapes: Vec<Matrix<u8>> = hexominoes
        .iter()
        .map(|h| h.clone().map(|x| (x != 0) as u8))
        .collect();
    for (i, cur_small) in all_smalls.iter().enumerate() {
        if let Some(pos) = shapes.iter().position(|x| x == cur_small) {
            return (sideways, size, i, pos);
        }
    }
    panic!("could not find matching hexomino");
}

fn get_adjacent_face(face: u8, facing: (isize, isize)) -> (u8, u8) {
    match (face, facing) {
        (1, directions::N) => (4, 0),
        (1, directions::E) => (3, 0),
        (1, directions::S) => (5, 0),
        (1, directions::W) => (2, 0),
        (2, directions::N) => (4, 1),
        (2, directions::E) => (1, 0),
        (2, directions::S) => (5, 3),
        (2, directions::W) => (6, 2),
        (3, directions::N) => (4, 3),
        (3, directions::E) => (6, 2),
        (3, directions::S) => (5, 1),
        (3, directions::W) => (1, 0),
        (4, directions::N) => (6, 2),
        (4, directions::E) => (3, 1),
        (4, directions::S) => (1, 0),
        (4, directions::W) => (2, 3),
        (5, directions::N) => (1, 0),
        (5, directions::E) => (3, 1),
        (5, directions::S) => (6, 2),
        (5, directions::W) => (2, 3),
        (6, directions::N) => (5, 0),
        (6, directions::E) => (3, 2),
        (6, directions::S) => (4, 0),
        (6, directions::W) => (2, 2),
        _ => panic!("unknown facing or face"),
    }
}

// fn linearize_hexomino(hexomino: &Matrix<u8>) {
//     hexomino.items().
// }

fn canonical_cube(board: &Matrix<char>, size: usize, hexomino: &Matrix<u8>) -> [Matrix<char>; 6] {
    let mut ret = [None, None, None, None, None, None];
    for ((row, col), val) in hexomino.items() {
        if *val == 0 {
            continue;
        }
        let dest = (*val / 10) as usize;
        let rots = (*val % 10) as usize;
        let mut slice = board
            .slice(
                (row * size)..((row + 1) * size),
                (col * size)..((col + 1) * size),
            )
            .unwrap();
        slice.rotate_cw(rots);
        ret[dest] = Some(slice);
    }
    ret.into_iter()
        .map(Option::unwrap)
        .collect::<Vec<Matrix<char>>>()
        .try_into()
        .unwrap()
}

#[test_case("inputs/example-22" => matches Ok(1061))]
#[test_case("inputs/input-22" => matches Ok(1061))]
pub fn puzzle2(filename: &str) -> Result<usize, std::io::Error> {
    #[rustfmt::skip]
    let hexominoes: Vec<Matrix<u8>> = vec![
        matrix![[23, 40, 31], [ 0, 10,  0], [ 0, 50,  0], [ 0, 60,  0]],
        matrix![[ 0, 40, 31], [20, 10,  0], [ 0, 50,  0], [ 0, 60,  0]],
        matrix![[ 0, 40, 31], [ 0, 10,  0], [21, 50,  0], [ 0, 60,  0]],
        matrix![[ 0, 40, 31], [ 0, 10,  0], [ 0, 50,  0], [22, 60,  0]],
        matrix![[ 0, 40,  0], [ 0, 10, 30], [21, 50,  0], [ 0, 60,  0]],
        matrix![[ 0, 40,  0], [20, 10, 30], [ 0, 50,  0], [ 0, 60,  0]],
        matrix![[ 0,  0, 43], [20, 10, 30], [ 0, 50,  0], [ 0, 60,  0]],
        matrix![[ 0,  0, 43], [ 0, 10, 30], [21, 50,  0], [ 0, 60,  0]],
        matrix![[ 0, 40, 31], [ 0, 10,  0], [21, 50,  0], [63,  0,  0]],
        matrix![[ 0,  0, 43], [ 0, 10, 30], [21, 50,  0], [63,  0,  0]],
        matrix![[ 0, 40,  0], [ 0, 10,  0], [21, 50,  0], [63,  0,  0], [33,  0,  0]],
    ];
    let (mut board, path) = parse_input(filename)?;
    let (sideways, size, flips, hexomino_idx) = find_hexamino(&board, &hexominoes);
    let hexomino = &hexominoes[hexomino_idx];
    if sideways {
        board.rotate_cw(1);
    }
    match flips {
        0 => (),
        1 => board.flip_lr(),
        2 => board.flip_ud(),
        3 => {
            board.flip_lr();
            board.flip_ud();
        }
        _ => panic!("unknown flips"),
    }
    let cube = canonical_cube(&board, size, hexomino);
    dbg!(sideways, size, flips, hexomino_idx);
    dbg!(&cube);
    Ok(0)
}
