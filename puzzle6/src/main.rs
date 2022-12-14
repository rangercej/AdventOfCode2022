use std::io::BufRead;

fn main() {
    let mut br = common::get_inputfilereader();

    let mut line = String::with_capacity(500);
    br.read_line(&mut line).unwrap();

    get_header_posn(&line);
    get_message_posn(&line);
}

fn get_header_posn(line: &str)
{
    let mut buf: Vec<char> = std::vec::Vec::new();
    let mut cnt = 0;
    let buflen = 4;

    for c in line.chars() {
        cnt += 1;
        append_buf(&mut buf, c, buflen);

        if buf.len() == buflen {
            if !contains_dupe(&buf) {
                break;
            }
        }
    }

    println!("Marker starts at {}", cnt);
}

fn get_message_posn(line: &str)
{
    let mut buf: Vec<char> = std::vec::Vec::new();
    let mut cnt = 0;
    let buflen = 14;

    for c in line.chars() {
        cnt += 1;
        append_buf(&mut buf, c, buflen);

        if buf.len() == buflen {
            if !contains_dupe(&buf) {
                break;
            }
        }
    }

    println!("Message starts at {}", cnt);
}

fn append_buf(buf: &mut Vec<char>, c: char, maxsize: usize)
{
    if buf.len() == maxsize {
        buf.remove(0);
    }

    buf.push(c);
}

fn contains_dupe(buf:&Vec<char>) -> bool
{
    let mut old = '*';
    let mut dup = false;

    let mut bufcopy = buf.clone();
    bufcopy.sort();
    for ch in bufcopy.iter() {
        if *ch == old {
            dup = true;
            break;
        }
        old = *ch;
    }

    dup
}