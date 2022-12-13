use std::io::BufRead;

fn main() {
    part1();
    part2();
}

fn part1() {
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

fn part2() {
    let br = common::get_inputfilereader();

    let mut total = 0;
    let mut group = std::vec::Vec::new();
    let mut n = 0;
 
    for line in br.lines() {
        let l = line.unwrap();
        group.push(l);
        n += 1;

        if n == 3 {
            n = 0;
            let result1 = intersect(&group[0], &group[1]);
            let result2 = intersect(&result1, &group[2]);

            // Sanity check
            if result2.len() == 0 {
                println!("No match for group:");
                println!("        {}", group[0]);
                println!("        {}", group[1]);
                println!("        {}", group[2]);
            }
 
            let c = result2.chars().nth(0).unwrap() as u32;

            // Finally! A-Z = 27-52 (from 65-90), a-z = 1-26 (from 97-122)
            let val = if c >= 92 { c - 96 } else { c - 65 + 27 };

            total += val;

            group.remove(0);
            group.remove(0);
            group.remove(0);
        }
    }

    println!("Total = {}", total);
}

fn intersect(s1: &String, s2: &String) -> String
{
    let mut result: String = std::string::String::new();

    for c1 in s1.chars() {
         for c2 in s2.chars() {
             if c1 == c2 {
                 result.push(c1);
             }
         }
     };

     result
}