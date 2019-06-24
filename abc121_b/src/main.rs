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
    let m: usize = read(&mut stdin_lock, &mut buf, b' ');
    let c: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut bs = vec![0i32; m];
    for i in 0..(m - 1) {
        bs[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    bs[m - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ans = 0;
    for _ in 0..n {
        let mut a = 0;
        for j in 0..(m - 1) {
            let aij: i32 = read(&mut stdin_lock, &mut buf, b' ');
            a += aij * bs[j];
        }
        let aij: i32 = read(&mut stdin_lock, &mut buf, b'\n');
        a += aij * bs[m - 1];
        a += c;
        if a > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
