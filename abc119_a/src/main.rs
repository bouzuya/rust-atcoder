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
    let s = read(&mut stdin_lock, &mut buf, b'\n');
    println!("{}", solve(s));
}

fn solve(s: String) -> &'static str {
    if s <= "2019/04/30".to_owned() {
        "Heisei"
    } else {
        "TBD"
    }
}

#[test]
fn example1() {
    assert_eq!("Heisei", solve("2019/04/30".to_owned()))
}

#[test]
fn example2() {
    assert_eq!("TBD", solve("2019/11/01".to_owned()))
}
