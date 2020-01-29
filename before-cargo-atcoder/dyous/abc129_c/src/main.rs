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
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut av = vec![true; n + 1];
    for _ in 0..m {
        av[read::<usize>(&mut stdin_lock, &mut buf, b'\n')] = false;
    }

    let mut tbl = vec![0i32; n + 1];
    tbl[0] = 1;
    for i in 1..n + 1 {
        let p2 = if i >= 2 && av[i - 2] { tbl[i - 2] } else { 0 };
        let p1 = if i >= 1 && av[i - 1] { tbl[i - 1] } else { 0 };
        tbl[i] = (p2 % 1_000_000_007 + p1 % 1_000_000_007) % 1_000_000_007;
    }
    print!("{}", tbl[n]);
}
