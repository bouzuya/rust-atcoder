fn read<T: std::str::FromStr>(stdin_lock: &mut std::io::StdinLock, buf: &mut Vec<u8>, delimiter: u8) -> T {
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
    let h = read(&mut stdin_lock, &mut buf, b' ');
    let w = read(&mut stdin_lock, &mut buf, b'\n');
    let h1 = read(&mut stdin_lock, &mut buf, b' ');
    let w1 = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(h, w, h1, w1));
}

fn solve(h: u16, w: u16, h1: u16, w1: u16) -> u16 {
    (h - h1) * (w - w1)
}

#[test]
fn example1() {
    assert_eq!(1, solve(3, 2, 2, 1))
}

#[test]
fn example2() {
    assert_eq!(6, solve(5, 5, 2, 3))
}

#[test]
fn example3() {
    assert_eq!(0, solve(2, 4, 2, 4))
}
