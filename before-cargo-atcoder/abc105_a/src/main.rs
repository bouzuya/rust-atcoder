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
    let n = read(&mut stdin_lock, &mut buf, b' ');
    let k = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(n, k));
}

fn solve(n: u8, k: u8) -> u8 {
    if n % k == 0 { 0 } else { 1 }
}

#[test]
fn example1() {
    assert_eq!(1, solve(7, 3))
}

#[test]
fn example2() {
    assert_eq!(0, solve(100, 10))
}

#[test]
fn example3() {
    assert_eq!(0, solve(1, 1))
}
