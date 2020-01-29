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
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let k: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut hv = vec![0i64; n];
    for i in 0..n {
        hv[i] = read(&mut stdin_lock, &mut buf, b'\n');
    }

    hv.sort();
    let mut ans = 1_000_000_000;
    for i in 0..n - k + 1 {
        ans = min(ans, hv[i + k - 1] - hv[i]);
    }
    println!("{}", ans);
}
