use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<(char, u64)>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input.lines().map(|x| {
        let (dir, count) = x.split_once(' ').unwrap();
        (dir.chars().next().unwrap(), count.parse().unwrap())
    }).collect())
}
#[derive(Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn do_move(dir: char, head: &mut Coord) {
    match dir {
        'U' => {
            head.y -= 1;
        },
        'D' => {
            head.y += 1;
        },
        'L' => {
            head.x -= 1;
        },
        'R' => {
            head.x += 1;
        },
        _ => panic!("unknown move direction"),
    }
}

fn fix_tail(head: &Coord, tail: &mut Coord) {
    let src = (head.x as i64 - tail.x as i64, head.y as i64 - tail.y as i64);
    let dst: (i64, i64) = match src {
        (2, 2) => (1, 1),
        (2, 1) => (1, 1),
        (2, 0) => (1, 0),
        (2, -1) => (1, -1),
        (2, -2) => (1, -1),
        (-2, 2) => (-1, 1),
        (-2, 1) => (-1, 1),
        (-2, 0) => (-1, 0),
        (-2, -1) => (-1, -1),
        (-2, -2) => (-1, -1),
        (1, 2) => (1, 1),
        (0, 2) => (0, 1),
        (-1, 2) => (-1, 1),
        (1, -2) => (1, -1),
        (0, -2) => (0, -1),
        (-1, -2) => (-1, -1),
        (1, 1) => (0, 0),
        (1, 0) => (0, 0),
        (1, -1) => (0, 0),
        (-1, 1) => (0, 0),
        (-1, 0) => (0, 0),
        (-1, -1) => (0, 0),
        (0, 1) => (0, 0),
        (0, 0) => (0, 0),
        (0, -1) => (0, 0),
        _ => panic!("wat"),
    };
    tail.x = (tail.x as i64 + dst.0) as usize;
    tail.y = (tail.y as i64 + dst.1) as usize;
    // if tail.y > head.y + 1 && tail.x 
    // if tail.y > head.y + 1 {
    //     tail.y = head.y + 1;
    //     if tail.x 
    //     tail.x = head.x;
    // }
    // if tail.y < head.y - 1 {
    //     tail.y = head.y - 1;
    //     tail.x = head.x;
    // }
    // if tail.x > head.x + 1 {
    //     tail.x = head.x + 1;
    //     tail.y = head.y;
    // }
    // if tail.x < head.x - 1 {
    //     tail.x = head.x - 1;
    //     tail.y = head.y;
    // }
}

#[test_case("inputs/example-09-1" => matches Ok(13))]
#[test_case("inputs/input-09" => matches Ok(6037))]
pub fn puzzle1(filename: &str) -> Result<u64, std::io::Error> {
    const SIZ: usize = 500;
    let input = parse_input(filename)?;
    let mut field = [[0; SIZ]; SIZ];
    let mut head = Coord { x: SIZ/2, y: SIZ/2 };
    let mut tail = Coord { x: SIZ/2, y: SIZ/2 };
    field[tail.y][tail.x] = 1;
    dbg!(&head, &tail);
    for (dir, count) in input {
        for _ in 0..count {
            do_move(dir, &mut head);
            fix_tail(&head, &mut tail);
            field[tail.y][tail.x] = 1;
            dbg!(&head, &tail);
        }
    }

    let res = field.iter().map(|x| x.iter().sum::<u64>()).sum();
    Ok(res)
}

#[test_case("inputs/example-09-2" => matches Ok(36))]
#[test_case("inputs/input-09" => matches Ok(2485))]
pub fn puzzle2(filename: &str) -> Result<u64, std::io::Error> {
    const SIZ: usize = 400;
    let input = parse_input(filename)?;
    let mut field = [[0; SIZ]; SIZ];
    let mut worm = vec![Coord { x: SIZ/2, y: SIZ/2 }; 10];
    field[worm[0].y][worm[0].x] = 1;
    for (dir, count) in input {
        for _ in 0..count {
            do_move(dir, &mut worm[0]);
            for i in 1..worm.len() {
                let head = worm[i-1];
                let mut tail = worm[i];
                fix_tail(&head, &mut tail);
                worm[i] = tail;
            }
            field[worm[worm.len()-1].y][worm[worm.len()-1].x] = 1;
            //dbg!(&worm);
        }
        // dbg!(field.iter().map(|x| x.iter().sum::<u64>()).sum::<u64>());
        // dbg!(&worm);
    }

    let res = field.iter().map(|x| x.iter().sum::<u64>()).sum();
    Ok(res)
}
