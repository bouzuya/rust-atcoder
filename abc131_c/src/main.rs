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
    let a: u64 = read(&mut stdin_lock, &mut buf, b' ');
    let b: u64 = read(&mut stdin_lock, &mut buf, b' ');
    let c: u64 = read(&mut stdin_lock, &mut buf, b' ');
    let d: u64 = read(&mut stdin_lock, &mut buf, b'\n');
    let n = b - a + 1;
    let cx = (n - (a % c) + (c - 1)) / c;
    let dx = (n - (a % d) + (d - 1)) / d;
    let cdx = (n - (a % (c * d)) + (c * d - 1)) / (c * d);
    let ans = n - cx - dx + cdx;
    println!("{}", ans);
}
