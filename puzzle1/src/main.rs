use std::io::prelude::*;

fn main() {
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
