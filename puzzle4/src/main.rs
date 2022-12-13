use std::io::BufRead;

fn main() {
    part1();
    part2();
}

fn part1() {
    let br = common::get_inputfilereader();

    let mut count = 0;
    for line in br.lines() {
        let l = line.unwrap();

        let sections: Vec<&str> = l.split(",").collect();
        let elf1: Vec<i32> = sections[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = sections[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        if elf1[0] <= elf2[0] && elf1[1] >= elf2[1] {
            count += 1;
        } else if elf1[0] >= elf2[0] && elf1[1] <= elf2[1] {
            count += 1;
        }
    }

    println!("There are {} sections fully included by the partner section", count);
}

fn part2() {
    let br = common::get_inputfilereader();

    let mut count = 0;
    for line in br.lines() {
        let l = line.unwrap();

        let sections: Vec<&str> = l.split(",").collect();
        let elf1: Vec<i32> = sections[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = sections[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        if elf1[0] <= elf2[1] && elf1[1] >= elf2[0] {
            count += 1;
        }
    }

    println!("There are {} overlapping sections", count);
}