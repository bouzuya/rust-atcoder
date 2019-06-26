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
    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');
    let mut wv = vec!["".to_owned(); n];
    for i in 0..n {
        wv[i] = read(&mut stdin_lock, &mut buf, b'\n');
    }
    let mut pc = wv[0].chars().last().unwrap();
    let mut ans = true;
    for i in 1..n {
        if !wv[i].starts_with(pc) {
            ans = false;
        }
        for j in 0..i {
            if &wv[j] == &wv[i] {
                ans = false;
            }
        }
        pc = wv[i].chars().last().unwrap();
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
