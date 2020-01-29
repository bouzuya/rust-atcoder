use std::cmp::min;

fn read<T: std::str::FromStr>(stdin_lock: &mut std::io::StdinLock, buf: &mut Vec<u8>, delimiter: u8) -> T {
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
    min(c, b / a)
}

#[test]
fn example1() {
    assert_eq!(4, solve(2, 11, 4))
}

#[test]
fn example2() {
    assert_eq!(3, solve(3, 9, 5))
}

#[test]
fn example3() {
    assert_eq!(0, solve(100, 1, 10))
}
