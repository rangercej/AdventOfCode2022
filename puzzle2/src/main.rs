use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let process_fullpath = std::env::current_exe().unwrap();
    let process_path = process_fullpath.parent().unwrap();

    let fpath = process_path.join("..\\..\\src\\input.txt");
    let fname = std::fs::canonicalize(fpath).unwrap();

    println!("Processing strategy in {}", fname.to_string_lossy());

    let file = match File::open(fname) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
            std::process::exit(1);
        }
    };

    let br = BufReader::new(file);

    // A == X == Rock == 1
    // B == Y == Paper == 2
    // C == Z == Scissors == 3
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

    let troll_score = match parts[0] {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        &_ => panic!("Unexpected troll score")
    };

    let elf_score = match parts[1] {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => panic!("Unexpected elf score")
    };

    return (troll_score, elf_score);
}