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
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut abv = vec![(0i64, 0i64); n];
    for i in 0..n {
        abv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        abv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    abv.sort();
    let mut count = m as i64;
    let mut ans = 0i64;
    for i in 0..n {
        let a = abv[i].0;
        let b = abv[i].1;
        if b <= count {
            ans += a * b;
            count -= b;
        } else {
            ans += a * count;
            count = 0;
        }
        if count == 0 {
            break;
        }
    }
    println!("{}", ans);
}
