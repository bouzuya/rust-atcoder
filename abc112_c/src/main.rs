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

    let mut ans = (0i32, 0i32, 0i32);
    for x in 0..100 + 1 {
        for y in 0..100 + 1 {
            let mut any = false;
            let mut h0 = 0;
            for i in 0..n {
                if xyhv[i].2 > 0 {
                    h0 = max((xyhv[i].0 - x).abs() + (xyhv[i].1 - y).abs() + xyhv[i].2, 0);
                    any = true;
                    break;
                }
            }
            if !any {
                continue;
            }

            let mut all = true;
            for i in 1..n {
                if xyhv[i].2 != max(h0 - ((xyhv[i].0 - x).abs() + (xyhv[i].1 - y).abs()), 0) {
                    all = false;
                    break;
                }
            }
            if all {
                ans = (x, y, h0);
            }
        }
    }
    println!("{} {} {}", ans.0, ans.1, ans.2);
}
