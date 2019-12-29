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
    let a: i64 = read(&mut stdin_lock, &mut buf, b' ');
    let b: i64 = read(&mut stdin_lock, &mut buf, b' ');
    let k: i64 = read(&mut stdin_lock, &mut buf, b'\n');

    if a >= k {
        println!("{} {}", a - k, b);
    } else if b >= k - a {
        println!("{} {}", 0, b - (k - a));
    } else {
        println!("{} {}", 0, 0);
    }
}
