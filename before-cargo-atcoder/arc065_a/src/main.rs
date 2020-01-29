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
    let s_rev: String = read::<String>(&mut stdin_lock, &mut buf, b'\n')
        .chars()
        .rev()
        .collect();
    let pv: Vec<String> = ["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|t| t.chars().rev().collect())
        .collect();
    let mut ans = true;
    let mut s = &s_rev[..];
    while s.len() > 0 {
        match pv.iter().find(|&p| s.starts_with(p)) {
            Some(p) => s = &s[p.len()..],
            None => {
                ans = false;
                break;
            }
        }
    }
    println!("{}", if ans { "YES" } else { "NO" });
}
