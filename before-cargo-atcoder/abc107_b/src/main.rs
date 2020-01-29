fn read<T: std::str::FromStr>(
    stdin_lock: &mut std::io::StdinLock,
    buf: &mut Vec<u8>,
    delimiter: u8,
) -> T {
    buf.clear();
    let l = std::io::BufRead::read_until(stdin_lock, delimiter, buf).unwrap();
    buf.truncate(l - 1); // remove delimiter
    let s = unsafe { std::str::from_utf8_unchecked(&buf) };
    s.parse().unwrap_or_else(|_| panic!("read"))
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let h: usize = read(&mut stdin_lock, &mut buf, b' ');
    let w: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut cv = vec!['.'; h * w];
    for i in 0..h {
        let l: String = read(&mut stdin_lock, &mut buf, b'\n');
        let cs: Vec<char> = l.chars().collect();
        for j in 0..w {
            cv[i * w + j] = cs[j];
        }
    }

    let mut skip_y: Vec<usize> = Vec::new();
    for y in 0..h {
        let mut has_black = false;
        for x in 0..w {
            if cv[y * w + x] == '#' {
                has_black = true;
            }
        }
        if !has_black {
            skip_y.push(y);
        }
    }

    let mut skip_x: Vec<usize> = Vec::new();
    for x in 0..w {
        let mut has_black = false;
        for y in 0..h {
            if cv[y * w + x] == '#' {
                has_black = true;
            }
        }
        if !has_black {
            skip_x.push(x);
        }
    }

    for y in 0..h {
        if skip_y.contains(&y) {
            continue;
        }
        for x in 0..w {
            if skip_x.contains(&x) {
                continue;
            }
            print!("{}", cv[y * w + x]);
        }
        println!();
    }
}
