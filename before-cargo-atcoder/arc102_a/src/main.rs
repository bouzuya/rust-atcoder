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

    let mut rv = vec![0i64; k];
    for x in 1..n + 1 {
        rv[x % k] += 1;
    }

    let mut ans = 0i64;
    for a in 0..k {
        let bc = (k - a) % k;
        if (bc + bc) % k == 0 {
            ans += rv[a] * rv[bc] * rv[bc];
        }
    }
    println!("{}", ans);
}
