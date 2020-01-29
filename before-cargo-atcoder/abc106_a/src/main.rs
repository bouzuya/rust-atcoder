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
    let a = read(&mut stdin_lock, &mut buf, b' ');
    let b = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(a, b));
}

fn solve(a: i16, b: i16) -> i16 {
    (a - 1) * (b - 1)
}

#[test]
fn example1() {
    assert_eq!(1, solve(2, 2))
}

#[test]
fn example2() {
    assert_eq!(24, solve(5, 7))
}
