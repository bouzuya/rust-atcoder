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
    let n: i64 = read(&mut stdin_lock, &mut buf, b'\n');
    let a: i64 = read(&mut stdin_lock, &mut buf, b'\n');
    let b: i64 = read(&mut stdin_lock, &mut buf, b'\n');
    let c: i64 = read(&mut stdin_lock, &mut buf, b'\n');
    let d: i64 = read(&mut stdin_lock, &mut buf, b'\n');
    let e: i64 = read(&mut stdin_lock, &mut buf, b'\n');

    let xv = vec![a, b, c, d, e];
    let min_x = xv.iter().min().unwrap();
    let ans = n / min_x + if n % min_x == 0 { 0 } else { 1 } + 4;
    println!("{}", ans);
}
