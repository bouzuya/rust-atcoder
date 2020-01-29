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
    let x = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(x));
}

fn solve(x: u8) -> String {
    if x == 7 || x == 5 || x == 3 {
        "YES".to_owned()
    } else {
        "NO".to_owned()
    }
}

#[test]
fn example1() {
    assert_eq!("YES", solve(5))
}

#[test]
fn example2() {
    assert_eq!("NO", solve(6))
}
