use std::io::Write;

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

    let n: i64 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut l = 1;
    let mut r = n;
    while r - l > 0 {
        let x = (r + l) / 2;
        println!("? {} {} {} {}", l, x, 1, n);
        std::io::stdout().flush().ok();
        let count: i64 = read(&mut stdin_lock, &mut buf, b'\n');
        if count == -1 {
            unreachable!();
        } else if count == x - l + 1 {
            l = x + 1;
        } else {
            r = x;
        }
    }
    let ans_i = r;

    let mut l = 1;
    let mut r = n;
    while r - l > 0 {
        let x = (r + l) / 2;
        println!("? {} {} {} {}", 1, n, l, x);
        std::io::stdout().flush().ok();
        let count: i64 = read(&mut stdin_lock, &mut buf, b'\n');
        if count == -1 {
            unreachable!();
        } else if count == x - l + 1 {
            l = x + 1;
        } else {
            r = x;
        }
    }
    let ans_j = r;
    println!("! {} {}", ans_i, ans_j);
}
