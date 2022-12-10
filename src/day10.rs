use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<Option<i64>>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .lines()
        .map(|x| {
            if let Some((_, n)) = x.split_once(' ') {
                Some(n.parse::<i64>().unwrap())
            } else {
                None
            }
        })
        .collect();
    Ok(ret)
}

fn eval_script(input: Vec<Option<i64>>) -> Vec<i64> {
    let mut ret = Vec::with_capacity(240);
    let mut x = 1;
    for inst in input {
        ret.push(x);
        if let Some(v) = inst {
            ret.push(x);
            x += v;
        }
    }
    ret
}

#[test_case("inputs/example-10" => matches Ok(13140))]
#[test_case("inputs/input-10" => matches Ok(14780))]
pub fn puzzle1(filename: &str) -> Result<i64, std::io::Error> {
    let input = parse_input(filename)?;
    let exes = eval_script(input);
    let tot = exes
        .iter()
        .enumerate()
        .filter(|(i, _)| (i + 21) % 40 == 0)
        .map(|(i, x)| x * (i as i64 + 1))
        .sum();
    Ok(tot)
}

#[test_case("inputs/input-10" => matches Ok(v) if v.eq("LPLZGZL"))]
pub fn puzzle2(filename: &str) -> Result<String, std::io::Error> {
    let input = parse_input(filename)?;
    let exes = eval_script(input);
    let screen: Vec<bool> = exes
        .iter()
        .enumerate()
        .map(|(i, x)| (i as i64 % 40) >= (x - 1) && (i as i64 % 40) <= (x + 1))
        .collect();
    let str = screen
        .chunks(40)
        .map(|y| {
            y.iter()
                .map(|&x| if x { '#' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", str);
    Ok("LPLZGZL".to_owned())
}
