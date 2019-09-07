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
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut bv = vec![0usize; n - 1];
    for i in 0..n - 2 {
        bv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    bv[n - 2] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut av = vec![0usize; n];
    av[0] = bv[0];
    for i in 1..n - 1 {
        av[i] = min(bv[i - 1], bv[i]);
    }
    av[n - 1] = bv[n - 2];

    let mut ans = 0;
    for i in 0..n {
        ans += av[i];
    }
    println!("{}", ans);
}
