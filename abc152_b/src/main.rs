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
    use std::cmp::min;
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();
    let a: u8 = read(&mut stdin_lock, &mut buf, b' ');
    let b: u8 = read(&mut stdin_lock, &mut buf, b'\n');
    let s: String = vec![('0' as u8 + a) as char; b as usize].into_iter().collect();
    let t: String = vec![('0' as u8 + b) as char; a as usize].into_iter().collect();
    println!("{}", min(s, t));
}
