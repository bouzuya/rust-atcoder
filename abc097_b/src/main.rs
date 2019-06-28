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
    let x: u32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ans = 1;
    for b in 1..x {
        for p in 2..x {
            let n = b.pow(p);
            if n > x {
                break;
            } else {
                ans = max(ans, n);
            }
        }
    }
    println!("{}", ans);
}
