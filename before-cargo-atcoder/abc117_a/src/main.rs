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
    let t = read(&mut stdin_lock, &mut buf, b' ');
    let x = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(t, x));
}

fn solve(t: i32, x: i32) -> String {
    format!("{0:.10}", f64::from(t) / f64::from(x))
}

#[test]
fn example1() {
    assert_eq!("2.6666666667", solve(8, 3))
}

#[test]
fn example2() {
    assert_eq!("99.0000000000", solve(99, 1))
}

#[test]
fn example3() {
    assert_eq!("0.0100000000", solve(1, 100))
}
