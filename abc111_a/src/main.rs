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

fn solve(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        chars[i] = match chars[i] {
            '1' => '9',
            '9' => '1',
            _ => unreachable!(),
        }
    }
    chars.into_iter().collect()
}

#[test]
fn example1() {
    assert_eq!("991", solve("119".to_owned()))
}

#[test]
fn example2() {
    assert_eq!("111", solve("999".to_owned()))
}
