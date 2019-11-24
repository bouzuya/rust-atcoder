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
    let a: i64 = read(&mut stdin_lock, &mut buf, b' ');
    let b: i64 = read(&mut stdin_lock, &mut buf, b' ');
    let x: i64 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ans = 0;
    let max_n = x / a;
    for i in 0..max_n {
        let n = max_n - i;
        let mut d = 0;
        let mut m = n;
        while m > 0 {
            d += 1;
            m /= 10;
        }
        let o = a * n + b * d;
        if o <= x {
            ans = min(1_000_000_000, n);
            break;
        }
    }
    println!("{}", ans);
}
