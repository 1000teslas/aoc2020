use petgraph::prelude::*;
use std::{collections::HashMap, fs};

fn f<'a, 'b: 'a, 'c: 'b>(
    s: &'b str,
    m: &mut HashMap<&'a str, u64>,
    g: &DiGraphMap<&'c str, u64>,
) -> u64 {
    match m.get(s) {
        None => {
            let mut sum = 0;
            for t in g.neighbors_directed(s, Outgoing) {
                sum += g.edge_weight(s, t).unwrap() * (f(t, m, g) + 1);
            }
            m.insert(s, sum);
            sum
        }
        Some(&v) => v,
    }
}
fn main() {
    let mut g = DiGraphMap::new();
    let s = fs::read_to_string("day7/input").unwrap();
    let mut m = HashMap::new();
    for l in s.lines() {
        let mut iter = l.split("contain");
        let outer = iter.next().unwrap().trim().trim_end_matches("bags").trim();
        for inner in iter.next().unwrap().split(",") {
            let trimmed = inner
                .trim()
                .trim_end_matches(|c| c == 's' || c == '.')
                .trim_end_matches("bag")
                .trim();
            if trimmed != "no other" {
                g.add_edge(outer, &trimmed[2..], trimmed[0..1].parse::<u64>().unwrap());
            }
        }
    }
    dbg!(f("shiny gold", &mut m, &g));
}
