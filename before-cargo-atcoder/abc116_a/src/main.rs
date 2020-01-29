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
    let ab = read(&mut stdin_lock, &mut buf, b' ');
    let bc = read(&mut stdin_lock, &mut buf, b' ');
    let ca = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(ab, bc, ca));
}

fn solve(ab: i32, bc: i32, _: i32) -> i32 {
    (ab * bc) / 2
}

#[test]
fn example1() {
    assert_eq!(6, solve(3, 4, 5))
}

#[test]
fn example2() {
    assert_eq!(30, solve(5, 12, 13))
}

#[test]
fn example3() {
    assert_eq!(630, solve(45, 28, 53))
}
