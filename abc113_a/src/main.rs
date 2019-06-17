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
    let x = read(&mut stdin_lock, &mut buf, b' ');
    let y = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(x, y));
}

fn solve(x: u8, y: u8) -> u8 {
    x + y / 2
}

#[test]
fn example1() {
    assert_eq!(110, solve(81, 58))
}

#[test]
fn example2() {
    assert_eq!(31, solve(4, 54))
}
