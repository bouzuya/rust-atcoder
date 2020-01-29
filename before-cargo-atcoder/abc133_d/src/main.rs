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

    let mut x1 = 0i64;
    let mut sign = 1;
    for i in 0..n {
        x1 += sign * av[i];
        sign *= -1;
    }

    let mut xv = vec![0i64; n];
    xv[0] = x1;
    for i in 0..n - 1 {
        xv[i + 1] = (av[i] - xv[i] / 2) * 2;
    }

    for i in 0..n - 1 {
        print!("{} ", xv[i]);
    }
    println!("{}", xv[n - 1]);
}
