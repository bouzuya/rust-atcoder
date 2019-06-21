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
    let mut vs = vec![0i16; n];
    for i in 0..(n - 1) {
        vs[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    vs[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');
    let mut cs = vec![0i16; n];
    for i in 0..(n - 1) {
        cs[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    cs[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ans = 0;
    for i in 0..n {
        if vs[i] - cs[i] > 0 {
            ans += vs[i] - cs[i];
        }
    }
    println!("{}", ans);
}
