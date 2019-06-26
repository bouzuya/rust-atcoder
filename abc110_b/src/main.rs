use std::cmp::{max, min};

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
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let m: usize = read(&mut stdin_lock, &mut buf, b' ');
    let x: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let y: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut xv = vec![0i32; n];
    for i in 0..(n - 1) {
        xv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    xv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');
    let mut yv = vec![0i32; m];
    for i in 0..(m - 1) {
        yv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    yv[m - 1] = read(&mut stdin_lock, &mut buf, b'\n');
    let max_x = *xv.iter().max().unwrap();
    let min_y = *yv.iter().min().unwrap();
    let ans = if max(x, max_x) < min(y, min_y) {
        "No War"
    } else {
        "War"
    };
    println!("{}", ans);
}
