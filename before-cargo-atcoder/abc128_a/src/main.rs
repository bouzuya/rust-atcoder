use std::cmp::min;
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
    let p = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(a, p));
}

fn solve(a: i32, p: i32) -> i32 {
    (a * 3 + p) / 2
}
