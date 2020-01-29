use std::cmp::max;
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
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cs: Vec<char> = s.chars().collect();
    let mut ans = 0;
    for i in 1..n {
        let set1: HashSet<_> = cs[0..i].iter().collect();
        let set2: HashSet<_> = cs[i..n].iter().collect();
        ans = max(ans, set1.intersection(&set2).count());
    }
    println!("{}", ans);
}
