use std::cmp::max;

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
    let cs: Vec<char> = s.chars().collect();
    let mut ans = 0;
    let mut count = 0;
    for i in cs.into_iter() {
        count = match i {
            'A' => count + 1,
            'C' => count + 1,
            'G' => count + 1,
            'T' => count + 1,
            _ => 0,
        };
        ans = max(ans, count);
    }
    println!("{}", ans);
}
