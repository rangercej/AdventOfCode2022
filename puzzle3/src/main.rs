use std::io::BufRead;
use array_tool::vec::Intersect;

fn main() {
    let br = common::get_inputfilereader();

    let mut total = 0;
    for line in br.lines() {
        let l = line.unwrap();
        let half_ln = l.chars().count() / 2;

        let contain1: Vec<char> = l[..half_ln].chars().collect();
        let contain2: Vec<char> = l[half_ln..].chars().collect();

        // Sanity check that string was properly halved.
        if contain1.len() != contain2.len() {
            panic!("Different lengths!");
        }

        // Note for future self: Vectors don't implement copy, so we need to
        // explicitly clone so we can dump contain2 into debug output later on.
        let result = contain1.intersect(contain2.clone());
        let c = result[0] as u32;

        // Finally! A-Z = 27-52 (from 65-90), a-z = 1-26 (from 97-122)
        let val = if c >= 92 { c - 96 } else { c - 65 + 27 };

        println!("{} = {} + {} => {}, {}, {}", l, 
            contain1.into_iter().collect::<String>(), 
            contain2.into_iter().collect::<String>(),
            result[0], c, val);

        total += val;
    }

    println!("Total = {}", total);
}
