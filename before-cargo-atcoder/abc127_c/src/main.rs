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
    let n: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut min_x = 1;
    let mut max_x = n;
    for _ in 0..m {
        let l: i32 = read(&mut stdin_lock, &mut buf, b' ');
        let r: i32 = read(&mut stdin_lock, &mut buf, b'\n');
        min_x = max(min_x, l);
        max_x = min(max_x, r);
    }
    let ans = if max_x < min_x { 0 } else { max_x - min_x + 1 };
    println!("{}", ans);
}
