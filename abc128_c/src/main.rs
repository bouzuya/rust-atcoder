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
    let n: u32 = read(&mut stdin_lock, &mut buf, b' ');
    let m: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut mv = vec![(0u16, 0u32); m];
    for i in 0..m {
        let mut s = 0u16;
        let k: usize = read(&mut stdin_lock, &mut buf, b' ');
        for _ in 0..k - 1 {
            s |= 1u16 << (read::<u16>(&mut stdin_lock, &mut buf, b' ') - 1);
        }
        s |= 1u16 << (read::<u16>(&mut stdin_lock, &mut buf, b'\n') - 1);
        mv[i].0 = s;
    }
    for i in 0..m - 1 {
        mv[i].1 = read(&mut stdin_lock, &mut buf, b' ');
    }
    mv[m - 1].1 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut ans = 0;
    for i in 0..2u16.pow(n) {
        if mv.iter().all(|m| ((i & m.0).count_ones() % 2) == m.1) {
            ans += 1
        }
    }
    println!("{}", ans);
}
