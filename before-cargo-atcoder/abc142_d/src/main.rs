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

fn gcd(a: i64, b: i64) -> i64 {
    fn gcd1(m: i64, n: i64) -> i64 {
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
    let a: i64 = read(&mut stdin_lock, &mut buf, b' ');
    let b: i64 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut m = gcd(a, b);

    let mut ans = 1;
    for i in 2..m {
        if i * i > m {
            break;
        }
        if m % i == 0 {
            ans += 1;
            while m % i == 0 {
                m = m / i;
            }
        }
    }
    if m > 1 {
        ans += 1;
    }
    println!("{}", ans);
}
