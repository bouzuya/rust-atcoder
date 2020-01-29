use std::cmp::min;
use std::iter::Iterator;

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
    let mut ws: Vec<i32> = Vec::new();
    for _ in 0..(n - 1) {
        ws.push(read(&mut stdin_lock, &mut buf, b' '));
    }
    ws.push(read(&mut stdin_lock, &mut buf, b'\n'));

    let sum: i32 = ws.iter().sum();
    let mut s1 = 0;
    let mut ans = sum;
    for w in ws {
        s1 += w;
        let s2 = sum - s1;
        ans = min(ans, (s1 - s2).abs());
    }
    println!("{}", ans);
}
