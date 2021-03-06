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
    let mut bv = vec![0; m];
    for _ in 0..n {
        let k: u8 = read(&mut stdin_lock, &mut buf, b' ');
        for _ in 0..(k - 1) {
            let a: usize = read(&mut stdin_lock, &mut buf, b' ');
            bv[a - 1] += 1;
        }
        let a: usize = read(&mut stdin_lock, &mut buf, b'\n');
        bv[a - 1] += 1;
    }
    let ans = bv.iter().filter(|x| **x == n).count();
    println!("{}", ans);
}
