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
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut xyhv = vec![(0i32, 0i32, 0i32); n];
    for i in 0..n {
        xyhv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        xyhv[i].1 = read(&mut stdin_lock, &mut buf, b' ');
        xyhv[i].2 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    for cx in 0..100 + 1 {
        for cy in 0..100 + 1 {
            let mut h = 0;
            for i in 0..n {
                let (xi, yi, hi) = xyhv[i];
                h = hi + (cx - xi).abs() + (cy - yi).abs();
                if h > 0 {
                    break;
                }
            }

            if (0..n).all(|i| {
                let (xi, yi, hi) = xyhv[i];
                hi == max(h - (cx - xi).abs() - (cy - yi).abs(), 0)
            }) {
                println!("{} {} {}", cx, cy, h);
                return;
            }
        }
    }

    unreachable!();
}
