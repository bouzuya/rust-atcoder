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
    let d: usize = read(&mut stdin_lock, &mut buf, b' ');
    let g: i64 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut pcv = vec![(0i64, 0i64); d];
    for i in 0..d {
        pcv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        pcv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut ans = 100 * 10;
    for bits in 0..(1 << d) {
        let mut x = 0i64;
        let mut y = 0;
        for i in 0..d {
            if (bits & (1 << i)) != 0 {
                x += (i as i64 + 1) * 100 * pcv[i].0 + pcv[i].1;
                y += pcv[i].0;
            }
        }
        if x >= g {
            ans = min(ans, y);
        } else {
            for i in 0..d {
                let j = d - i - 1;
                if (bits & (1 << j)) == 0 {
                    for _ in 0..pcv[j].0 {
                        x += (j as i64 + 1) * 100;
                        y += 1;
                        if x >= g {
                            ans = min(ans, y);
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
