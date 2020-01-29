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

fn f(n: i64, r: i64) -> i64 {
    let mut m = n;
    let mut s = 0;
    while m > 0 {
        s += m % r;
        m /= r;
    }
    s
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let n: i64 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ans = n;
    for i in 0..n + 1 {
        ans = min(ans, f(i, 6) + f(n - i, 9));
    }
    println!("{}", ans);
}
