use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let mut br = common::get_inputfilereader();
    let mut stacks = get_stacks(&mut br);

    // Dump stacks
    for stack in stacks.clone() {
        let s: String = stack.into_iter().collect();
        println!("{}", s);
    }

    move_stacks(&mut br, &mut stacks);

    let mut output = String::new();
    for mut stack in stacks {
        let c = stack.pop().unwrap();
        output.push(c);
    }

    println!("Top of stacks are {}", output);
}

fn get_stacks(br: &mut BufReader<File>) -> Vec<Vec<char>>
{
    let mut stacks = Vec::new();

    // Create empty stacks for crates
    for _i in 1..10 {
        let stack: Vec<char> = Vec::new();
        stacks.push(stack);
    }

    // Process map
    let mut linecount = 0;
    for line in br.lines() {
        linecount += 1;
        println!("Processing line {}", linecount);
        let l = line.unwrap();

        if l.is_empty() {
            break;
        }

        // Get existing crate map
        let mut posn = 0;
        while posn < l.len() {
            let ixend = if posn+4 > l.len() { l.len() } else { posn+4 };
            let cr = &l[posn..ixend];
            let cid = cr.chars().nth(1).unwrap();
            
            posn += 4;
         
            if cid as u32 != 0x20 {
                let stacknum = (posn / 4) - 1;
                stacks[stacknum].insert(0, cid);
            }
        }
    }

    stacks
}

fn move_stacks(br: &mut BufReader<File>, stacks: &mut Vec<Vec<char>>) {
    for line in br.lines() {
        // Instruction is "move <count> from <src> to <dest>"
        let l = line.unwrap();
        let parts: Vec<&str> = l.split(' ').collect();

        let count: usize = parts[1].parse::<usize>().unwrap();
        let src: usize = parts[3].parse::<usize>().unwrap();
        let dest: usize = parts[5].parse::<usize>().unwrap();

        for _i in 0..count {
            let c = stacks[src-1].pop().unwrap();
            stacks[dest-1].push(c);
        }
    }
}