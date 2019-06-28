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
    let t: String = read(&mut stdin_lock, &mut buf, b'\n');
    let sc: Vec<char> = s.chars().collect();
    let mut ans = false;
    for i in 0..sc.len() {
        let mut tc: Vec<char> = Vec::new();
        for j in i..sc.len() {
            tc.push(sc[j]);
        }
        for j in 0..i {
            tc.push(sc[j]);
        }
        let t2: String = tc.into_iter().collect();
        if t2 == t {
            ans = true;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
