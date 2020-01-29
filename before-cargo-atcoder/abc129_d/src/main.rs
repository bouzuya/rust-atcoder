use std::cmp::max;

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
    let mut sv = Vec::new();
    for _ in 0..h {
        let s: String = read(&mut stdin_lock, &mut buf, b'\n');
        let xv: Vec<char> = s.chars().collect();
        sv.push(xv);
    }

    let mut xv = vec![vec![0i32; w]; h];
    for y in 0..h {
        let mut c = 0;
        for x in 0..w {
            if sv[y][x] == '.' {
                c += 1;
            } else {
                c = 0;
            }
            xv[y][x] = c;
        }
        for x in 1..w {
            let rx = w - x - 1;
            if xv[y][rx] != 0 && xv[y][rx] < xv[y][rx + 1] {
                xv[y][rx] = xv[y][rx + 1];
            }
        }
    }

    let mut yv = vec![vec![0i32; w]; h];
    for x in 0..w {
        let mut c = 0;
        for y in 0..h {
            if sv[y][x] == '.' {
                c += 1;
            } else {
                c = 0;
            }
            yv[y][x] = c;
        }
        for y in 1..h {
            let ry = h - y - 1;
            if yv[ry][x] != 0 && yv[ry][x] < yv[ry + 1][x] {
                yv[ry][x] = yv[ry + 1][x];
            }
        }
    }

    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            ans = max(ans, xv[y][x] + yv[y][x]);
        }
    }
    println!("{}", if ans > 1 { ans - 1 } else { ans });
}
