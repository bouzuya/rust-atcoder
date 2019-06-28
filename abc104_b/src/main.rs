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
    let ans = cs[0] == 'A'
        && cs[2..(cs.len() - 1)].iter().filter(|&&c| c == 'C').count() == 1
        && (cs[1].is_lowercase()
            && cs[2..(cs.len() - 1)]
                .iter()
                .all(|&c| c == 'C' || c.is_lowercase())
            && cs[cs.len() - 1].is_lowercase());
    println!("{}", if ans { "AC" } else { "WA" });
}
