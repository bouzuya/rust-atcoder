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
    let mut av = vec![0i64; n];
    for i in 0..n - 1 {
        av[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    av[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut bv = vec![0i64; n];
    for i in 0..n {
        bv[i] = av[i] - (i as i64 + 1);
    }
    bv.sort();
    let m = if n % 2 == 0 {
        (bv[n / 2 - 1] + bv[n / 2]) / 2
    } else {
        bv[n / 2]
    };
    let mut ans = 0i64;
    for i in 0..n {
        ans += (av[i] - (m + (i as i64 + 1))).abs();
    }
    println!("{}", ans);
}
