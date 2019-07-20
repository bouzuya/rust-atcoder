use std::cmp::max;
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

    let mut lv = vec![0i32; n];
    lv[0] = av[0];
    for i in 1..n {
        lv[i] = gcd(lv[i - 1], av[i]);
    }
    let mut rv = vec![0i32; n];
    rv[n - 1] = av[n - 1];
    for i in 1..n {
        let j = n - i - 1;
        rv[j] = gcd(rv[j + 1], av[j]);
    }

    let mut ans = max(gcd(av[1], rv[1]), gcd(lv[n - 2], av[n - 2]));
    for i in 1..n - 1 {
        ans = max(ans, gcd(lv[i - 1], rv[i + 1]));
    }
    println!("{}", ans);
}
