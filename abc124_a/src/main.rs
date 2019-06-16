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
    let a = read(&mut stdin_lock, &mut buf, b' ');
    let b = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(a, b));
}

fn solve(a: u8, b: u8) -> u8 {
    if a == b {
        a + b
    } else if a > b {
        a + a - 1
    } else {
        b + b - 1
    }
}

#[test]
fn example1() {
    assert_eq!(9, solve(5, 3))
}

#[test]
fn example2() {
    assert_eq!(7, solve(3, 4))
}

#[test]
fn example3() {
    assert_eq!(12, solve(6, 6))
}
