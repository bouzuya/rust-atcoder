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
    let a: usize = read(&mut stdin_lock, &mut buf, b' ');
    let b: usize = read(&mut stdin_lock, &mut buf, b' ');
    let c: usize = read(&mut stdin_lock, &mut buf, b' ');
    let x: usize = read(&mut stdin_lock, &mut buf, b' ');
    let y: usize = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ans = c * max(x * 2, y * 2);
    for cx in 0..max(x * 2, y * 2) {
        let ax = if x > cx / 2 { x - cx / 2 } else { 0 };
        let bx = if y > cx / 2 { y - cx / 2 } else { 0 };
        ans = min(ans, a * ax + b * bx + c * cx);
    }
    println!("{}", ans);
}
