#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    println!("D1.1: {:?}", day1::part1(day1::INPUT));
    println!("D1.2: {:?}", day1::part2(day1::INPUT));
    println!("D2.1: {:?}", day2::part1(day2::INPUT));
    println!("D2.2: {:?}", day2::part2(day2::INPUT));
    println!("D3.1: {:?}", day3::part1(day3::INPUT));
    println!("D3.2: {:?}", day3::part2(day3::INPUT));
    println!("D4.1: {:?}", day4::part1(day4::INPUT));
    println!("D4.2: {:?}", day4::part2(day4::INPUT));
    println!("D5.1: {:?}", day5::part1(day5::INPUT));
    println!("D5.2: {:?}", day5::part2(day5::INPUT));
    println!("D6.1: {:?}", day6::part1(day6::INPUT));
    println!("D6.2: {:?}", day6::part2(day6::INPUT));
}
