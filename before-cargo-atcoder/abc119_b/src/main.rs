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
    let mut y: f64 = 0f64;
    for _ in 0..n {
        let x: f64 = read(&mut stdin_lock, &mut buf, b' ');
        let u: String = read(&mut stdin_lock, &mut buf, b'\n');
        y += x * if u == "JPY" {
            1f64
        } else if u == "BTC" {
            380000f64
        } else {
            unreachable!()
        };
    }
    println!("{}", y);
}
