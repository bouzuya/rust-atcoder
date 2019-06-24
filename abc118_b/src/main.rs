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
    let mut bits: u32 = 0xffffffff;
    for _ in 0..n {
        let k: u8 = read(&mut stdin_lock, &mut buf, b' ');
        let mut mask: u32 = 0;
        for _ in 0..(k - 1) {
            let a: u8 = read(&mut stdin_lock, &mut buf, b' ');
            mask |= 1 << (a - 1);
        }
        let a: u8 = read(&mut stdin_lock, &mut buf, b'\n');
        mask |= 1 << (a - 1);
        bits &= mask;
    }
    let mut ans = 0;
    for i in 0..m {
        if ((bits >> i) & 1) == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
