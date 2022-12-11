use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let process_fullpath = std::env::current_exe().unwrap();
    let process_path = process_fullpath.parent().unwrap();

    let fpath = process_path.join("..\\..\\src\\input.txt");
    let fname = std::fs::canonicalize(fpath).unwrap();

    println!("Processing elf distribution in {}", fname.to_string_lossy());

    let file = match File::open(fname) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
            std::process::exit(1);
        }
    };

    let br = BufReader::new(file);

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
