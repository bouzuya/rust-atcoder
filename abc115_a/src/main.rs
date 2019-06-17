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
    let d = read(&mut stdin_lock, &mut buf, b' ');
    println!("{}", solve(d));
}

fn solve(d: u8) -> String {
    if d == 25 {
        "Christmas".to_owned()
    } else if d == 24 {
        "Christmas Eve".to_owned()
    } else if d == 23 {
        "Christmas Eve Eve".to_owned()
    } else if d == 22 {
        "Christmas Eve Eve Eve".to_owned()
    } else {
        unreachable!()
    }
}

#[test]
fn example1() {
    assert_eq!("Christmas", solve(25))
}

#[test]
fn example2() {
    assert_eq!("Christmas Eve Eve Eve", solve(22))
}
