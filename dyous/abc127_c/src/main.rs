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
    let n: i64 = read(&mut stdin_lock, &mut buf, b' ');
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut lrv = vec![(0i64, 0i64); m];
    for i in 0..m {
        lrv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        lrv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut l: i64 = 1;
    let mut r: i64 = n;
    for i in 0..m {
        l = max(l, lrv[i].0);
        r = min(r, lrv[i].1);
    }
    let ans = max(0, r - l + 1);
    println!("{}", ans);
}
