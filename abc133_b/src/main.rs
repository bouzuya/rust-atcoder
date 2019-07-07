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
    let d: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut xvv = vec![vec![0i32; d]; n];
    for i in 0..n {
        for j in 0..d - 1 {
            xvv[i][j] = read(&mut stdin_lock, &mut buf, b' ');
        }
        xvv[i][d - 1] = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let mut s = 0;
            for k in 0..d {
                s += (xvv[i][k] - xvv[j][k]).pow(2);
            }
            let a: f64 = f64::from(s);
            let b: f64 = a.sqrt().trunc();;
            if b * b == a {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
