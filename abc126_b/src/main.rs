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
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let cs: Vec<char> = s.chars().collect();
    let s1: String = vec![cs[0], cs[1]].into_iter().collect();
    let s2: String = vec![cs[2], cs[3]].into_iter().collect();
    let u1: u8 = s1.parse().unwrap();
    let u2: u8 = s2.parse().unwrap();
    if 1 <= u1 && u1 <= 12 {
        if 1 <= u2 && u2 <= 12 {
            println!("AMBIGUOUS");
        } else {
            println!("MMYY");
        }
    } else {
        if 1 <= u2 && u2 <= 12 {
            println!("YYMM");
        } else {
            println!("NA");
        }
    }
}
