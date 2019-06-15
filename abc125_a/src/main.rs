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
    let b = read(&mut stdin_lock, &mut buf, b' ');
    let t = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(a, b, t));
}

fn solve(a: u16, b: u16, t: u16) -> u16 {
    let mut count = 0;
    for i in 1..(t + 1) {
        if (i % a) == 0 {
            count += b
        }
    }
    count
}

#[test]
fn example1() {
    assert_eq!(10, solve(3, 5, 7))
}

#[test]
fn example2() {
    assert_eq!(6, solve(3, 2, 9))
}

#[test]
fn example3() {
    assert_eq!(0, solve(20, 20, 19))
}

#[test]
fn my_example() {
    assert_eq!(400, solve(1, 20, 20))
}
