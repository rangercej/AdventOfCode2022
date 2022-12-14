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

    for c in line.chars() {
        cnt += 1;
        append_buf(&mut buf, c, 4);

        if buf.len() == 4 {
            if buf[0] == buf[1] || buf[0] == buf[2] || buf[0] == buf[3]
                        || buf[1] == buf[2] || buf[1] == buf[3]
                        || buf[2] == buf[3] {
                    continue;
            } else {
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

    for c in line.chars() {
        cnt += 1;
        append_buf(&mut buf, c, 14);

        if buf.len() == 14 {
            let mut old = '*';
            let mut dup = false;
            
            // Need a nicer way of doing this other than
            // cloning the vector on each iteration. Feels
            // icky. Works, but icky. The "if" chain in
            // get_header_posn() only really practical for
            // short lengths.
            let mut bufcopy = buf.clone();
            bufcopy.sort();
            for ch in bufcopy.iter() {
                if *ch == old {
                    dup = true;
                    break;
                }
                old = *ch;
            }

            if !dup {
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
