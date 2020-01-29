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
    let mut pv = vec![0i32; n];
    for i in 0..n - 1 {
        pv[i] = read(&mut stdin_lock, &mut buf, b' ');
    }
    pv[n - 1] = read(&mut stdin_lock, &mut buf, b'\n');

    let mut qv = pv.clone();
    qv.sort();
    let mut count = 0;
    for i in 0..n {
        if pv[i] != qv[i] {
            count += 1;
        }
    }
    let ans = if count == 0 || count == 2 {
        "YES"
    } else {
        "NO"
    };
    println!("{}", ans);
}
