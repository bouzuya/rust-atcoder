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
    let a: i32 = read(&mut stdin_lock, &mut buf, b' ');
    let b: i32 = read(&mut stdin_lock, &mut buf, b'\n');
    let mut ans: i32 = 0;
    for i in 1..n + 1 {
        let mut m = i;
        let mut s = 0;
        while m != 0 {
            s += m % 10;
            m /= 10;
        }
        if a <= s && s <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}
