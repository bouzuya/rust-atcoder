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
    let k = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(k));
}

fn solve(a: i32) -> i32 {
    (a / 2) * (a / 2 + if a % 2 == 0 { 0 } else { 1 })
}

#[test]
fn example1() {
    assert_eq!(2, solve(3))
}

#[test]
fn example2() {
    assert_eq!(9, solve(6))
}

#[test]
fn example3() {
    assert_eq!(30, solve(11))
}

#[test]
fn example4() {
    assert_eq!(625, solve(50))
}
