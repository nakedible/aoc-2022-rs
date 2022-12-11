use test_case::test_case;

#[derive(Debug)]
enum Operation {
    Plus(u64),
    Mul(u64),
    Pow,
}

#[derive(Debug)]
struct Monkey<T> {
    items: Vec<T>,
    op: Operation,
    test: u64,
    true_dest: usize,
    false_dest: usize,
    inspections: u64,
}

fn parse_input(filename: &str) -> Result<Vec<Monkey<u64>>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let mut monkeys = Vec::new();
    for monkey_input in input.split("\n\n") {
        let mut monkey_lines = monkey_input.lines();
        monkey_lines.next();
        let items_line = monkey_lines.next().unwrap();
        let (_, items_list) = items_line.split_once(": ").unwrap();
        let items: Vec<u64> = items_list.split(", ").map(|x| x.parse().unwrap()).collect();
        let (_, op_line) = monkey_lines.next().unwrap().split_once("= old ").unwrap();
        let op = match op_line.split_once(' ').unwrap() {
            ("*", "old") => Operation::Pow,
            ("*", op_val) => Operation::Mul(op_val.parse().unwrap()),
            ("+", op_val) => Operation::Plus(op_val.parse().unwrap()),
            _ => panic!("invalid input"),
        };
        let (_, divisible) = monkey_lines
            .next()
            .unwrap()
            .split_once("divisible by ")
            .unwrap();
        let test = divisible.parse().unwrap();
        let (_, true_dest_str) = monkey_lines
            .next()
            .unwrap()
            .split_once("throw to monkey ")
            .unwrap();
        let true_dest = true_dest_str.parse().unwrap();
        let (_, false_dest_str) = monkey_lines
            .next()
            .unwrap()
            .split_once("throw to monkey ")
            .unwrap();
        let false_dest = false_dest_str.parse().unwrap();
        monkeys.push(Monkey {
            items,
            op,
            test,
            true_dest,
            false_dest,
            inspections: 0,
        });
    }
    Ok(monkeys)
}

#[test_case("inputs/input-11" => matches Ok(55216))]
pub fn puzzle1(filename: &str) -> Result<u64, std::io::Error> {
    let mut monkeys = parse_input(filename)?;
    for _round in 1..=20 {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            for mut item in items {
                monkeys[i].inspections += 1;
                match monkeys[i].op {
                    Operation::Plus(v) => item += v,
                    Operation::Mul(v) => item *= v,
                    Operation::Pow => item *= item,
                }
                item /= 3;
                if (item % monkeys[i].test) == 0 {
                    let dest = monkeys[i].true_dest;
                    monkeys[dest].items.push(item);
                } else {
                    let dest = monkeys[i].false_dest;
                    monkeys[dest].items.push(item);
                }
            }
        }
    }
    let mut inspections: Vec<u64> = monkeys.iter().map(|x| x.inspections).collect();
    inspections.sort();
    let ret = inspections
        .into_iter()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap();
    Ok(ret)
}

#[test_case("inputs/input-11" => matches Ok(12848882750))]
pub fn puzzle2(filename: &str) -> Result<u64, std::io::Error> {
    let monkeys = parse_input(filename)?;
    let mods: Vec<u64> = monkeys.iter().map(|x| x.test).collect();
    let mut monkeys: Vec<Monkey<Vec<u64>>> = monkeys
        .into_iter()
        .map(|monkey| {
            let items: Vec<Vec<u64>> = monkey
                .items
                .iter()
                .map(|item| mods.iter().map(|m| item % m).collect())
                .collect();
            Monkey {
                items,
                op: monkey.op,
                test: monkey.test,
                true_dest: monkey.true_dest,
                false_dest: monkey.false_dest,
                inspections: monkey.inspections,
            }
        })
        .collect();
    for _round in 1..=10000 {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            for mut item in items {
                monkeys[i].inspections += 1;
                for (j, m) in mods.iter().enumerate() {
                    item[j] = match monkeys[i].op {
                        Operation::Plus(v) => (item[j] + v) % m,
                        Operation::Mul(v) => (item[j] * v) % m,
                        Operation::Pow => (item[j] * item[j]) % m,
                    }
                }
                if item[i] == 0 {
                    let dest = monkeys[i].true_dest;
                    monkeys[dest].items.push(item);
                } else {
                    let dest = monkeys[i].false_dest;
                    monkeys[dest].items.push(item);
                }
            }
        }
    }
    let mut inspections: Vec<u64> = monkeys.iter().map(|x| x.inspections).collect();
    inspections.sort();
    let ret = inspections
        .into_iter()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap();
    Ok(ret)
}
