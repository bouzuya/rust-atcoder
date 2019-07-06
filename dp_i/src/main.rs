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
    let mut pv = vec![0f64; n];
    for i in 0..n - 1 {
        pv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    pv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut tbl = vec![vec![0f64; n + 1]; n + 1];
    tbl[0][0] = 1f64;
    for i in 1..n + 1 {
        for j in 0..i {
            tbl[i][j] += tbl[i - 1][j] * (1f64 - pv[i - 1]);
            tbl[i][j + 1] += tbl[i - 1][j] * pv[i - 1];
        }
    }

    let mut ans = 0f64;
    for i in ((n + 1) / 2)..(n + 1) {
        ans += tbl[n][i];
    }
    println!("{}", ans);
}
