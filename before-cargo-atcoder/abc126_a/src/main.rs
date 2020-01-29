use std::ascii::AsciiExt;
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
    let n = read(&mut stdin_lock, &mut buf, b' ');
    let k = read(&mut stdin_lock, &mut buf, b'\n');
    let s = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(n, k, s));
}

fn solve(_: u8, k: u8, s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    for c in chars.iter_mut() {
        if i == k - 1 {
            *c = c.to_ascii_lowercase();
        }
        i += 1;
    }
    chars.into_iter().collect()
}

#[test]
fn example1() {
    assert_eq!("aBC", solve(3, 1, "ABC".to_owned()))
}

#[test]
fn example2() {
    assert_eq!("CAbA", solve(4, 3, "CABA".to_owned()));
}
