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
    let n: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let y: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    for a in 0..n + 1 {
        let ya = 10000 * a;
        if ya > y {
            continue;
        }
        for b in 0..(n - a) + 1 {
            if ya + 5000 * b + 1000 * (n - a - b) == y {
                println!("{} {} {}", a, b, (n - a - b));
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}
