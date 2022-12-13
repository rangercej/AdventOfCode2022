use std::io::prelude::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let br = common::get_inputfilereader();

    let mut elf = 1;
    let mut total = 0;
    let mut max_tot = 0;
    let mut max_elf = 0;

    for line in br.lines() {
        let l = line.unwrap();
        if l.is_empty() {
            if total > max_tot {
                max_elf = elf;
                max_tot = total;
            }

            elf = elf+1;
            total = 0;
        } else {
            total = total + l.parse::<i32>().unwrap();
        }
    }

    if total > max_tot {
        max_elf = elf;
        max_tot = total;
    }

    println!("Elf {} carrying {} calories", max_elf, max_tot);

}

fn part2() {
    let br = common::get_inputfilereader();

    let mut calories: Vec<i32> = std::vec::Vec::new();
    let mut total = 0;

    for line in br.lines() {
        let l = line.unwrap();
        if l.is_empty() {
            calories.push(total);
            total = 0;
        } else {
            total = total + l.parse::<i32>().unwrap();
        }
    }

    calories.push(total);
    calories.sort();

    let len = calories.len();
    
    println!("Total cals from top 3 elves = {}", calories[len-1] + calories[len-2] + calories[len-3]);
}
