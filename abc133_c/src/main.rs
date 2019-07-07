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
    let l: usize = read(&mut stdin_lock, &mut buf, b' ');
    let r: usize = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ans = 2019;
    for i in 0..2019 {
        for j in i + 1..2019 {
            let x = ((l + i) * (l + j)) % 2019;
            if x < ans && l + j <= r {
                ans = x;
            }
        }
    }
    println!("{}", ans);
}
