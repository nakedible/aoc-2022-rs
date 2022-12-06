use test_case::test_case;

#[derive(Debug)]
struct Move {
    count: u64,
    from: usize,
    to: usize,
}

fn parse_input(filename: &str) -> Result<(Vec<Vec<char>>, Vec<Move>), std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let (stackpart, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = Vec::with_capacity(9);
    for line in stackpart.lines() {
        for stack in 0..=8 {
            let idx = 1 + (stack * 4);
            let chars: Vec<char> = line.chars().collect();
            if chars.len() <= idx {
                break;
            }
            if chars[idx] == '1' {
                break;
            }
            if chars[idx] == ' ' {
                continue;
            }
            while stacks.len() <= stack {
                stacks.push(Vec::with_capacity(9));
            }
            stacks[stack].insert(0, chars[idx]);
        }
    }

    let moves = moves
        .lines()
        .map(|x| {
            let mut s = x.split_whitespace();
            s.next();
            let count = s.next().unwrap().parse::<u64>().unwrap();
            s.next();
            let from = s.next().unwrap().parse::<usize>().unwrap();
            s.next();
            let to = s.next().unwrap().parse::<usize>().unwrap();
            Move { count, from, to }
        })
        .collect();

    Ok((stacks, moves))
}

fn do_move(stacks: &mut [Vec<char>], action: &Move) {
    for _idx in 0..(action.count) {
        let elem = stacks[action.from - 1].pop().unwrap();
        stacks[action.to - 1].push(elem);
    }
}

#[test_case("inputs/input-05" => matches Ok(v) if v.eq("LBLVVTVLP"))]
pub fn puzzle1(filename: &str) -> Result<String, std::io::Error> {
    let (mut stacks, moves) = parse_input(filename)?;
    for action in moves {
        do_move(&mut stacks, &action);
    }
    let ret: String = stacks.iter().map(|x| x.last().unwrap()).collect();
    Ok(ret)
}

fn do_move_stack(stacks: &mut [Vec<char>], action: &Move) {
    let fromlast = stacks[action.from - 1].len();
    let tail: Vec<char> = stacks[action.from - 1]
        .drain((fromlast - (action.count as usize))..fromlast)
        .collect();
    stacks[action.to - 1].extend(tail);
}

#[test_case("inputs/input-05" => matches Ok(v) if v.eq("TPFFBDRJD"))]
pub fn puzzle2(filename: &str) -> Result<String, std::io::Error> {
    let (mut stacks, moves) = parse_input(filename)?;
    for action in moves {
        do_move_stack(&mut stacks, &action);
    }
    let ret: String = stacks.iter().map(|x| x.last().unwrap()).collect();
    Ok(ret)
}
