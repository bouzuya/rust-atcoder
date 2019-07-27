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

fn gcd(a: i32, b: i32) -> i32 {
    fn gcd1(m: i32, n: i32) -> i32 {
        if n == 0 {
            m
        } else {
            gcd1(n, m % n)
        }
    }
    if a > b {
        gcd1(a, b)
    } else {
        gcd1(b, a)
    }
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

    let mut ans = av[0];
    for i in 1..n {
        ans = gcd(ans, av[i]);
    }
    println!("{}", ans);
}
