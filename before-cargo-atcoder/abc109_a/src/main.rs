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

fn solve(a: u8, b: u8) -> String {
    if (a % 2 == 0) || (b % 2 == 0) {
        "No".to_owned()
    } else {
        "Yes".to_owned()
    }
}

#[test]
fn example1() {
    assert_eq!("Yes", solve(3, 1))
}

#[test]
fn example2() {
    assert_eq!("No", solve(1, 2))
}

#[test]
fn example3() {
    assert_eq!("No", solve(2, 2))
}
