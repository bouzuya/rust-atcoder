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
    let mut pv = vec![(0i32, 0i32, 0i32); n + 1];
    for i in 1..n + 1 {
        pv[i].0 = read(&mut stdin_lock, &mut buf, b' ');
        pv[i].1 = read(&mut stdin_lock, &mut buf, b' ');
        pv[i].2 = read(&mut stdin_lock, &mut buf, b'\n');
    }

    let mut ans = true;
    for i in 1..n + 1 {
        let dt = pv[i].0 - pv[i - 1].0;
        let dx = (pv[i].1 - pv[i - 1].1).abs();
        let dy = (pv[i].2 - pv[i - 1].2).abs();
        if dx + dy > dt || (dt - (dx + dy)) % 2 != 0 {
            ans = false;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
