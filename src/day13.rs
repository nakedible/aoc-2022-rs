use serde_json::{from_str, json, to_string, Value};
use test_case::test_case;

fn parse_input(filename: &str) -> Result<Vec<(Value, Value)>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    let mut ret = Vec::new();
    for pair in input.split("\n\n") {
        let (left, right) = pair.split_once('\n').unwrap();
        ret.push((from_str(left)?, from_str(right)?));
    }
    Ok(ret)
}

fn cmp_packets(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => match a.as_u64().unwrap().cmp(&b.as_u64().unwrap())
        {
            std::cmp::Ordering::Less => Some(true),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(false),
        },
        (Value::Array(_), Value::Number(r)) => {
            cmp_packets(left, &Value::Array(vec![Value::Number(r.clone())]))
        }
        (Value::Number(l), Value::Array(_)) => {
            cmp_packets(&Value::Array(vec![Value::Number(l.clone())]), right)
        }
        (Value::Array(a), Value::Array(b)) => {
            for i in 0..std::cmp::max(a.len(), b.len()) {
                if i >= a.len() {
                    return Some(true);
                }
                if i >= b.len() {
                    return Some(false);
                }
                if let Some(r) = cmp_packets(&a[i], &b[i]) {
                    return Some(r);
                }
            }
            None
        }
        _ => panic!("invalid input"),
    }
}

#[test_case("inputs/input-13" => matches Ok(5580))]
pub fn puzzle1(filename: &str) -> Result<usize, std::io::Error> {
    let input = parse_input(filename)?;
    let ret = input
        .iter()
        .enumerate()
        .map(|(i, (a, b))| {
            if cmp_packets(a, b).unwrap_or(true) {
                i + 1
            } else {
                0
            }
        })
        .sum();
    Ok(ret)
}

#[test_case("inputs/input-13" => matches Ok(26200))]
pub fn puzzle2(filename: &str) -> Result<usize, std::io::Error> {
    let input = parse_input(filename)?;
    let mut input: Vec<Value> = input
        .into_iter()
        .flat_map(|tup| std::iter::once(tup.0).chain(std::iter::once(tup.1)))
        .chain(std::iter::once(json!([[2]])))
        .chain(std::iter::once(json!([[6]])))
        .collect();
    input.sort_by(|a, b| match cmp_packets(a, b) {
        Some(true) => std::cmp::Ordering::Less,
        None => std::cmp::Ordering::Equal,
        Some(false) => std::cmp::Ordering::Greater,
    });
    let as_strings: Vec<String> = input.iter().map(|x| to_string(x).unwrap()).collect();
    let a = as_strings.iter().position(|x| x == "[[2]]").unwrap();
    let b = as_strings.iter().position(|x| x == "[[6]]").unwrap();
    Ok((a + 1) * (b + 1))
}
