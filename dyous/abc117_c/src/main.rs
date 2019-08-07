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
    let mut xv = vec![0i32; m];
    for i in 0..m - 1 {
        xv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    xv[m - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    if n >= m {
        println!("{}", 0);
    } else {
        xv.sort();

        let mut yv = vec![0i32; m];
        yv[0] = 0;
        for i in 1..m {
            yv[i] = xv[i] - xv[i - 1];
        }
        yv.sort();

        let mut ans = 0;
        for i in 0..m - n + 1 {
            ans += yv[i];
        }
        println!("{}", ans);
    }
}
