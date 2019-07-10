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
    tbl[1] = tbl[0];
    for i in 2..n + 1 {
        let a = if av[i - 1] { tbl[i - 1] } else { 0 };
        let b = if av[i - 2] { tbl[i - 2] } else { 0 };
        tbl[i] = (a + b) % 1_000_000_007;
    }
    println!("{}", tbl[n]);
}
