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
    let s: u16 = read(&mut stdin_lock, &mut buf, b'\n');
    let u1: u16 = s / 100;
    let u2: u16 = s % 100;
    let ym = 1 <= u2 && u2 <= 12;
    let my = 1 <= u1 && u1 <= 12;
    let ans = match (ym, my) {
        (true, true) => "AMBIGUOUS",
        (true, false) => "YYMM",
        (false, true) => "MMYY",
        (false, false) => "NA",
    };
    println!("{}", ans);
}
