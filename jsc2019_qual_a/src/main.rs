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
    let m: usize = read(&mut stdin_lock, &mut buf, b' ');
    let d: usize = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ans = 0;
    for i in 1..m + 1 {
        for j in 1..d + 1 {
            let d1 = j % 10;
            let d10 = j / 10;
            if d1 >= 2 && d10 >= 2 && i == d1 * d10 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
