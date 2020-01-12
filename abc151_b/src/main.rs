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
    let k: usize = read(&mut stdin_lock, &mut buf, b' ');
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut av = vec![0usize; n - 1];
    for i in 0..n - 2 {
        av[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    av[n - 2] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut sum = 0;
    for i in 0..av.len() {
        sum += av[i];
    }
    if sum > m * n {
        println!("0");
    } else {
        let x = m * n - sum;
        if x > k {
            println!("-1");
        } else {
            println!("{}", x);
        }
    }
}
