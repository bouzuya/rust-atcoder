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
    let x: u32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut mv = vec![0u32; n];
    for i in 0..n {
        mv[i] = read(&mut stdin_lock, &mut buf, b'\n');
    }
    let sum_m = mv.iter().sum::<u32>();
    let min_m = mv.iter().min().unwrap();
    let ans = n as u32 + (x - sum_m) / min_m;
    println!("{}", ans);
}
