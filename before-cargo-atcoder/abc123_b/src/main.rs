use std::cmp::min;

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
    let mut hs = vec![0i16; 5];
    hs[0] = read(&mut stdin_lock, &mut buf, b'\n');
    hs[1] = read(&mut stdin_lock, &mut buf, b'\n');
    hs[2] = read(&mut stdin_lock, &mut buf, b'\n');
    hs[3] = read(&mut stdin_lock, &mut buf, b'\n');
    hs[4] = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ans = 130 * 5;
    for i in 0..5 {
        let mut a = 0;
        a += hs[i];
        for j in 0..i {
            a += hs[j] / 10 * 10 + if hs[j] % 10 == 0 { 0 } else { 10 };
        }
        for j in (i + 1)..5 {
            a += hs[j] / 10 * 10 + if hs[j] % 10 == 0 { 0 } else { 10 };
        }
        ans = min(ans, a);
    }
    println!("{}", ans);
}
