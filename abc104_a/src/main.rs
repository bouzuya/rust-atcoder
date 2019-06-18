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
    let r = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(r));
}

fn solve(r: u16) -> String {
    if r < 1200 {
        "ABC".to_owned()
    } else if r < 2800 {
        "ARC".to_owned()
    } else {
        "AGC".to_owned()
    }
}
