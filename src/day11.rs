use test_case::test_case;

#[derive(Debug)]
enum Operation {
    Plus(u64),
    Mul(u64),
    Pow,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: u64,
    true_dest: usize,
    false_dest: usize,
}

fn parse_input(filename: &str) -> Result<Vec<Monkey>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let mut monkeys = Vec::new();
    for (i, monkey_input) in input.split("\n\n").enumerate() {
        let mut monkey_lines = monkey_input.lines();
        monkey_lines.next();
        let items_line = monkey_lines.next().unwrap();
        let (_, items_list) = items_line.split_once(": ").unwrap();
        let items: Vec<u64> = items_list.split(", ").map(|x| x.parse::<u64>().unwrap()).collect();
        let (_, op_line) = monkey_lines.next().unwrap().split_once("= old ").unwrap();
        let op = match op_line.split_once(' ').unwrap() {
            ("*", "old") => Operation::Pow,
            ("*", op_val) => Operation::Mul(op_val.parse().unwrap()),
            ("+", op_val) => Operation::Plus(op_val.parse().unwrap()),
            _ => panic!("invalid input"),
        };
        let (_, divisible) = monkey_lines.next().unwrap().split_once("divisible by ").unwrap();
        let test = divisible.parse().unwrap();
        let (_, true_dest_str) = monkey_lines.next().unwrap().split_once("throw to monkey ").unwrap();
        let true_dest = true_dest_str.parse().unwrap();
        let (_, false_dest_str) = monkey_lines.next().unwrap().split_once("throw to monkey ").unwrap();
        let false_dest = false_dest_str.parse().unwrap();
        monkeys.push(Monkey {
            items,
            op,
            test,
            true_dest,
            false_dest,
        });
    }
    Ok(monkeys)
}

#[test_case("inputs/input-11" => matches Ok(14780))]
pub fn puzzle1(filename: &str) -> Result<i64, std::io::Error> {
    let input = parse_input(filename)?;
    dbg!(&input);
    Ok(0)
}
