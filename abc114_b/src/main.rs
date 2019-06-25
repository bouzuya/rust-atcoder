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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ans = 999;
    let cs: Vec<char> = s.chars().collect();
    let l = 3;
    for i in 0..(cs.len() - l + 1) {
        let t: String = cs.iter().skip(i).take(l).collect();
        let x: i32 = t.parse().unwrap();
        ans = min(ans, (x - 753).abs());
    }
    println!("{}", ans);
}
