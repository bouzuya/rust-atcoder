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
    let mut mv = vec![0u32; m];
    for i in 0..m {
        let k: usize = read(&mut stdin_lock, &mut buf, b' ');
        let mut bits = 0;
        for _ in 0..k - 1 {
            bits |= 1 << (read::<u16>(&mut stdin_lock, &mut buf, b' ') - 1);
        }
        bits |= 1 << (read::<u16>(&mut stdin_lock, &mut buf, b'\n') - 1);
        mv[i] = bits;
    }
    let mut pv = vec![0u32; m];
    for i in 0..m - 1 {
        pv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    pv[m - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let ans = (0..1 << n)
        .filter(|i| (0..m).all(|j| ((mv[j] & i).count_ones() % 2) == pv[j]))
        .count();
    println!("{}", ans);
}
