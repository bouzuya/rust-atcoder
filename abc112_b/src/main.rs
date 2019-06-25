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
    let n: usize = read(&mut stdin_lock, &mut buf, b' ');
    let t: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ctv = vec![(0, 0); n];
    for i in 0..n {
        ctv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        ctv[i].1 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    ctv.sort();
    ctv.iter()
        .filter(|&&(_, t1)| t1 <= t)
        .nth(0)
        .map_or_else(|| println!("TLE"), |&(c, _)| println!("{}", c));
}
