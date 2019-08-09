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
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let k: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut xv = vec![0i64; n];
    for i in 0..n - 1 {
        xv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    xv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut yv = vec![0i64; n];
    for i in 0..n {
        yv[i] = (xv[i] - xv[0]).abs();
    }

    let mut ans = xv[0].abs() + (xv[n - 1] - xv[0]).abs();
    for i in 0..n - k + 1 {
        ans = min(
            ans,
            min(
                xv[i].abs() + (xv[i + k - 1] - xv[i]).abs(),
                (xv[i + k - 1] - xv[i]).abs() + xv[i + k - 1].abs(),
            ),
        );
    }
    println!("{}", ans);
}
