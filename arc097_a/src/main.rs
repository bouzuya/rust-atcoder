use std::cmp::min;
use std::collections::HashSet;

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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let k: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut set = HashSet::new();

    for i in 0..s.len() {
        for j in i..min(s.len(), i + k) {
            set.insert(&s[i..j + 1]);
        }
    }

    let mut v: Vec<&str> = set.into_iter().collect();
    v.sort();
    let ans = v[k - 1];
    println!("{}", ans);
}
