use std::cmp::max;

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
    let n: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut suml = 0;
    let mut maxl = 0;
    for _ in 0..(n - 1) {
        let l: i32 = read(&mut stdin_lock, &mut buf, b' ');
        suml += l;
        maxl = max(maxl, l);
    }
    let l: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    suml += l;
    maxl = max(maxl, l);
    let ans = if suml - maxl > maxl { "Yes" } else { "No" };
    println!("{}", ans);
}
