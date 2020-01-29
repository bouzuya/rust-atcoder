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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let a: u64 = read(&mut stdin_lock, &mut buf, b' ');
    let b: u64 = read(&mut stdin_lock, &mut buf, b' ');
    let c: u64 = read(&mut stdin_lock, &mut buf, b' ');
    let d: u64 = read(&mut stdin_lock, &mut buf, b'\n');
    let p = a - 1;
    let px = p - (p / c) - (p / d) + (p / lcm(c, d));
    let bx = b - (b / c) - (b / d) + (b / lcm(c, d));
    let ans = bx - px;
    println!("{}", ans);
}
