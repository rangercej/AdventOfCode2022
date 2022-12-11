use std::io::prelude::*;

pub fn run()
{
    let br = common::get_inputfilereader();

    // 0 = lost, 3 = draw, 6 = win

    let mut total_score = 0;
    for line in br.lines() {
        let l = line.unwrap();
        let score = get_score(l);

        total_score += score.1;
        if score.0 == score.1 {
            total_score += 3;
        } else if score.1 == score.0 + 1 {
            total_score += 6;
        } else if score.1 == 1 && score.0 == 3 {
            total_score += 6;
        };
    };

    println!("Total score = {}", total_score);

}

fn get_score(line: String) -> (i32, i32)
{
    let tokens = line.split(" ");
    let parts: Vec<&str> = tokens.collect();

    // A == Rock == 1   [rock beats scissors] [rock loses paper]
    // B == Paper == 2  [paper beats rock] [paper loses scissors]
    // C == Scissors == 3 [scissors beats paper] [scissors loses rock]
    // X == Lose, Y == Draw, Z == Win

    let troll_score = match parts[0] {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        &_ => panic!("Unexpected troll score")
    };

    let elf_score = match parts[1] {
        "X" => match troll_score {
            1 => 3,
            2 => 1,
            3 => 2,
            _ => unreachable!()
        }
        "Y" => troll_score,
        "Z" => match troll_score {
            1 => 2,
            2 => 3,
            3 => 1,
            _ => unreachable!()
        }
        &_ => panic!("Unexpected elf score")
    };

    return (troll_score, elf_score);
}