use std::cmp::min;

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
    let a: usize = read(&mut stdin_lock, &mut buf, b' ');
    let b: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cv: Vec<char> = s.chars().collect();
    let n = cv.len();
    let ans = n == a + b + 1
        && cv[a] == '-'
        && cv[0..a].iter().all(|&c| c.is_digit(10))
        && cv[min(a + 1, n - 1)..n].iter().all(|&c| c.is_digit(10));
    println!("{}", if ans { "Yes" } else { "No" });
}
