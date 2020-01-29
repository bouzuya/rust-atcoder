use std::io::{BufRead, StdinLock};
use std::str::FromStr;

fn read<T: FromStr>(stdin_lock: &mut StdinLock, buf: &mut Vec<u8>, delimiter: u8) -> T {
    buf.clear();
    let l = stdin_lock.read_until(delimiter, buf).unwrap();
    buf.truncate(l - 1); // remove delimiter
    let s = unsafe { std::str::from_utf8_unchecked(&buf) };
    s.parse().unwrap_or_else(|_| panic!("read"))
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let a = read(&mut stdin_lock, &mut buf, b'\n');
    let b = read(&mut stdin_lock, &mut buf, b'\n');
    let c = read(&mut stdin_lock, &mut buf, b'\n');
    let d = read(&mut stdin_lock, &mut buf, b'\n');
    let e = read(&mut stdin_lock, &mut buf, b'\n');
    let k = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(a, b, c, d, e, k));
}

fn solve(a: u8, _: u8, _: u8, _: u8, e: u8, k: u8) -> &'static str {
    if e - a <= k {
        "Yay!"
    } else {
        ":("
    }
}

#[test]
fn example1() {
    assert_eq!("Yay!", solve(1, 2, 4, 8, 9, 15))
}

#[test]
fn example2() {
    assert_eq!(":(", solve(15, 18, 26, 35, 36, 18))
}
