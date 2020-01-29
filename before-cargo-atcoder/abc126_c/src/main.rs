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
    let n: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let k: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ans = 0f64;
    for i in 1..n + 1 {
        let mut j = 0;
        let mut s = 1;
        while s * i < k {
            s *= 2;
            j += 1;
        }
        ans += 1.0 / (f64::from(1 << j) * f64::from(n));
    }
    println!("{:.12}", ans);
}
