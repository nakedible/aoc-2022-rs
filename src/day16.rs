use std::collections::HashMap;

use pathfinding::directed::dijkstra::dijkstra_all;
use test_case::test_case;

#[derive(Debug)]
struct Valve {
    name: &'static str,
    rate: i64,
    leads_to: Vec<&'static str>,
}

fn parse_input(filename: &str) -> Result<Vec<Valve>, std::io::Error> {
    let input = std::fs::read_to_string(filename)?;
    Ok(input
        .lines()
        .map(|l| {
            let mut words = l.splitn(10, ' ');
            let name = words.nth(1).unwrap();
            let rate = words.nth(2).unwrap();
            let leads_to = words.nth(4).unwrap().split(", ");
            Valve {
                name: Box::leak(name.to_owned().into_boxed_str()),
                rate: rate[5..rate.len() - 1].parse::<i64>().unwrap(),
                leads_to: leads_to
                    .map(|x| &*Box::leak(x.to_owned().into_boxed_str()))
                    .collect(),
            }
        })
        .collect())
}

struct CostEntry {
    rate: i64,
    leads_to: HashMap<&'static str, i64>,
}

fn build_costs_map(input: Vec<Valve>) -> HashMap<&'static str, CostEntry> {
    let input_map: HashMap<&str, Valve> = input.into_iter().map(|i| (i.name, i)).collect();
    let costs_map: HashMap<&str, CostEntry> = input_map
        .values()
        .map(|i| {
            let x: HashMap<&str, i64> = dijkstra_all(&i.name, |n| {
                input_map[n]
                    .leads_to
                    .iter()
                    .map(|&x| (x, 1))
                    .collect::<Vec<(&str, i64)>>()
            })
            .iter()
            .map(|(&k, (_, cost))| (k, *cost))
            .collect();
            (
                i.name,
                CostEntry {
                    rate: i.rate,
                    leads_to: x,
                },
            )
        })
        .collect();
    costs_map
}

fn find_best(
    costs_map: &HashMap<&'static str, CostEntry>,
    pos: &'static str,
    rem: i64,
    flow: i64,
    visited: &mut Vec<&'static str>,
) -> i64 {
    let CostEntry { rate: _, leads_to } = &costs_map[pos];
    let mut best = flow;
    for (next, cost) in leads_to {
        if visited.contains(next) {
            continue;
        }
        let rate = costs_map[next].rate;
        if rate == 0 {
            continue;
        }
        let rem_left = rem - (cost + 1);
        if rem_left <= 0 {
            continue;
        }
        visited.push(next);
        let new_rate = find_best(costs_map, next, rem_left, flow + (rem_left * rate), visited);
        if new_rate > best {
            best = new_rate;
        }
        visited.pop();
    }
    best
}

#[test_case("inputs/example-16" => matches Ok(1651))]
#[test_case("inputs/input-16" => matches Ok(1724))]
pub fn puzzle1(filename: &str) -> Result<i64, std::io::Error> {
    let input = parse_input(filename)?;
    let costs_map = build_costs_map(input);
    let ret = find_best(&costs_map, "AA", 30, 0, &mut Vec::new());
    Ok(ret)
}
