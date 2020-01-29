use std::cmp::{max, min};

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
    let mut pv = vec![0i32; n];
    for i in 0..n - 1 {
        pv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    pv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ans = 0;
    for i in 1..n - 1 {
        let max_p = max(pv[i - 1], pv[i + 1]);
        let min_p = min(pv[i - 1], pv[i + 1]);
        if min_p < pv[i] && pv[i] < max_p {
            ans += 1;
        }
    }
    println!("{}", ans);
}
