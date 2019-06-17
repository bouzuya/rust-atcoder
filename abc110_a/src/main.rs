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
    let a = read(&mut stdin_lock, &mut buf, b' ');
    let b = read(&mut stdin_lock, &mut buf, b' ');
    let c = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(a, b, c));
}

fn solve(a: u8, b: u8, c: u8) -> u8 {
    max(max(a * 10 + b + c, a + b * 10 + c), a + b + c * 10)
}

#[test]
fn example1() {
    assert_eq!(53, solve(1, 5, 2))
}

#[test]
fn example2() {
    assert_eq!(108, solve(9, 9, 9))
}

#[test]
fn example3() {
    assert_eq!(82, solve(6, 6, 7))
}
