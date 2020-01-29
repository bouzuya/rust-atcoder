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
    let w: f64 = read(&mut stdin_lock, &mut buf, b' ');
    let h: f64 = read(&mut stdin_lock, &mut buf, b' ');
    let x: f64 = read(&mut stdin_lock, &mut buf, b' ');
    let y: f64 = read(&mut stdin_lock, &mut buf, b'\n');
    let a = (w * h) / 2f64;
    let b = if (w / 2f64) == x && (h / 2f64) == y { 1 } else { 0 };
    print!("{:.6} {}", a, b);
}
