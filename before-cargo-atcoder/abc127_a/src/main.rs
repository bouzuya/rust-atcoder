use std::io::{BufRead, StdinLock};

fn read(stdin_lock: &mut StdinLock, buf: &mut Vec<u8>, delimiter: u8) -> i32 {
    buf.clear();
    let l = stdin_lock.read_until(delimiter, buf).unwrap();
    buf.truncate(l - 1); // remove delimiter
    let s = unsafe { std::str::from_utf8_unchecked(&buf) };
    s.parse().unwrap()
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let a = read(&mut stdin_lock, &mut buf, b' ');
    let b = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(a, b));
}

fn solve(a: i32, b: i32) -> i32 {
    if a >= 13 {
        b
    } else if a >= 6 {
        b / 2
    } else {
        0
    }
}

#[test]
fn example1() {
    assert_eq!(100, solve(30, 100))
}

#[test]
fn example2() {
    assert_eq!(50, solve(12, 100))
}

#[test]
fn example3() {
    assert_eq!(0, solve(0, 100))
}
