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
    let mut vv = vec![0f64; n];
    for i in 0..n - 1 {
        vv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    vv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    vv.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut ans = vv[0];
    for i in 1..n {
        ans = (ans + vv[i]) / 2f64;
    }
    println!("{}", ans);
}
