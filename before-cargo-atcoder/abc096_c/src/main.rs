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
    let mut sv: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let s: String = read(&mut stdin_lock, &mut buf, b'\n');
        sv.push(s.chars().collect());
    }

    let mut ans = true;
    for y in 0..h {
        for x in 0..w {
            if sv[y][x] == '#' {
                let t = if y < 1 { false } else { sv[y - 1][x] == '#' };
                let l = if x < 1 { false } else { sv[y][x - 1] == '#' };
                let r = if x + 1 >= w {
                    false
                } else {
                    sv[y][x + 1] == '#'
                };
                let b = if y + 1 >= h {
                    false
                } else {
                    sv[y + 1][x] == '#'
                };
                if !(t || l || r || b) {
                    ans = false;
                }
            }
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
