mod day2;

fn main() {
    println!("D2.1: {:?}", day2::part1(day2::INPUT));
    println!("D2.2: {:?}", day2::part2(day2::INPUT));
}

fn tr(s: &str, from: &str, to: &str) -> String {
    let c1: Vec<char> = from.chars().collect();
    let c2: Vec<char> = to.chars().collect();
    let mut ns = String::new();
    for c in s.chars() {
        if let Some(idx) = c1.iter().position(|&x| x == c) {
            if idx < c2.len() {
                ns.push(c2[idx]);
            }
        } else {
            ns.push(c);
        }
    }
    ns
}

fn split(input: &str, on: &str, from: &str, to: &str) -> Vec<String> {
    tr(input, from, to)
        .split(on)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
}

use std::iter::FromIterator;

fn splitto<Y: std::str::FromStr>(input: &str, on: &str, from: &str, to: &str) -> Vec<Y>
where
    <Y as std::str::FromStr>::Err: std::fmt::Debug,
{
    Vec::from_iter(
        split(input, on, from, to)
            .iter()
            .map(|s| s.parse::<Y>().unwrap()),
    )
}
