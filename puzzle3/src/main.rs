use std::io::BufRead;

fn main() {
    let br = common::get_inputfilereader();

    let mut total = 0;
    for line in br.lines() {
        let l = line.unwrap();
        let half_ln = l.chars().count() / 2;

        let contain1 = &l[..half_ln];
        let contain2 = &l[half_ln..];

        // Sanity check that string was properly halved.
        if contain1.len() != contain2.len() {
            panic!("Different lengths!");
        }

       let mut c = 0;
       'outer: for c1 in contain1.chars() {
            for c2 in contain2.chars() {
                if c1 == c2 {
                    c = c1 as u32;
                    break 'outer;
                }
            }
        };

        // Finally! A-Z = 27-52 (from 65-90), a-z = 1-26 (from 97-122)
        let val = if c >= 92 { c - 96 } else { c - 65 + 27 };

        total += val;
    }

    println!("Total = {}", total);
}
