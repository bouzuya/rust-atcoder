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
    let mut av = vec![0i64; n + 1];
    for i in 0..n {
        av[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    av[n] = read(&mut stdin_lock, &mut buf, b'\n');
    let mut bv = vec![0i64; n];
    for i in 0..n - 1 {
        bv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    bv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut cv = vec![0i64; n + 1];
    for i in 0..n {
        let j = n - i - 1;
        let p = if bv[j] >= av[j + 1] - cv[j + 1] {
            av[j + 1] - cv[j + 1]
        } else {
            bv[j]
        };
        let c = if bv[j] - p >= av[j] { av[j] } else { bv[j] - p };
        cv[j + 1] += p;
        cv[j] += c;
    }

    let ans: i64 = cv.into_iter().sum();
    println!("{}", ans);
}
