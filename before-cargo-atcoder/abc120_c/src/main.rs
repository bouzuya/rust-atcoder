use std::cmp::min;

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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');

    let cv: Vec<char> = s.chars().collect();
    let mut rc = 0;
    let mut bc = 0;
    for i in 0..cv.len() {
        match cv[i] {
            '0' => rc += 1,
            '1' => bc += 1,
            _ => unreachable!(),
        }
    }
    let ans = min(rc, bc) * 2;
    println!("{}", ans);
}
