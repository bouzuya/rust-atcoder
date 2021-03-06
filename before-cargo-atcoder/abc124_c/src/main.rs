use std::cmp::min;

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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cv: Vec<char> = s.chars().collect();

    let ans = match cv.len() {
        1 => 0,
        2 => {
            if cv[0] == cv[1] {
                1
            } else {
                0
            }
        }
        n => {
            let mut c1 = 0; // 010101...
            let mut c2 = 0; // 101010...
            for i in 0..n {
                if cv[i] == '0' {
                    if i % 2 == 0 {
                        c2 += 1;
                    } else {
                        c1 += 1;
                    }
                } else if cv[i] == '1' {
                    if i % 2 == 0 {
                        c1 += 1;
                    } else {
                        c2 += 1;
                    }
                } else {
                    unreachable!();
                }
            }
            min(c1, c2)
        }
    };
    println!("{}", ans);
}
