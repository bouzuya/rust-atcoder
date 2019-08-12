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
    let mut n: i32 = read(&mut stdin_lock, &mut buf, b'\n');

    let mut cv = Vec::new();
    if n == 0 {
        cv.push('0');
    } else {
        while n != 0 {
            if n % 2 == 0 {
                cv.push('0');
                n = n / -2;
            } else {
                cv.push('1');
                n = (n - 1) / -2;
            }
        }
        cv.reverse();
    }
    let ans = cv.iter().map(|&c| c).collect::<String>();
    println!("{}", ans);
}
