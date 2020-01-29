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
    let b = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(b));
}

fn solve(a: String) -> &'static str {
    match a.as_str() {
        "A" => "T",
        "C" => "G",
        "G" => "C",
        "T" => "A",
        _ => unreachable!()
    }
}

#[test]
fn example1() {
    assert_eq!("T", solve("A".to_owned()))
}

#[test]
fn example2() {
    assert_eq!("C", solve("G".to_owned()))
}
