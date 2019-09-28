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
    let mut av = vec![0i32; n];
    for i in 0..n - 1 {
        av[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    av[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut sum = 0;
    let mut dv = vec![0i32; n + 2];
    for i in 0..n + 1 {
        let d = (if i == n { 0 } else { av[i] } - if i == 0 { 0 } else { av[i - 1] }).abs();
        sum += d;
        dv[i + 1] = d;
    }
    for i in 0..n {
        println!(
            "{}",
            sum - (dv[i + 1] + dv[i + 2])
                + (if i == n - 1 { 0 } else { av[i + 1] } - if i == 0 { 0 } else { av[i - 1] })
                    .abs()
        );
    }
}
