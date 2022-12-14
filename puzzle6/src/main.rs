use std::io::BufRead;

fn main() {
    let mut br = common::get_inputfilereader();

    let mut line = String::with_capacity(500);
    br.read_line(&mut line).unwrap();

    let header_pos = get_unique_posn(&line, 4);
    let message_pos = get_unique_posn(&line, 14);

    println!("Header position = {}", header_pos);
    println!("Message position = {}", message_pos);
}

fn get_unique_posn(line: &str, buflen: usize) -> usize
{
    let mut buf: Vec<char> = vec!['*'; buflen];
    let mut cnt = 0;

    for c in line.chars() {
        buf[cnt%buflen] = c;

        cnt += 1;
    
        if cnt >= buflen && !contains_dupe(&buf) {
            break;
        }
    }

    cnt
}

fn contains_dupe(buf:&Vec<char>) -> bool
{
    let mut chars = String::new();

    for ch in buf.iter() {
        if chars.contains(*ch) {
            return true;
        }
        chars.push(*ch);
    }

    false
}