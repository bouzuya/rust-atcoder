use std::io::*;

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

fn is_male(s: &str) -> Option<bool> {
    match s {
        "Vacant" => None,
        "Male" => Some(true),
        "Female" => Some(false),
        _ => unreachable!(),
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut buf: Vec<u8> = Vec::new();

    let n: usize = read(&mut stdin_lock, &mut buf, b'\n');

    println!("{}", 0);
    std::io::stdout().flush().ok();
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let s_0 = is_male(s.as_str());
    if s_0.is_none() {
        return;
    }

    println!("{}", n - 1);
    std::io::stdout().flush().ok();
    let s: String = read(&mut stdin_lock, &mut buf, b'\n');
    let s_n1 = is_male(s.as_str());
    if s_n1.is_none() {
        return;
    }

    let mut l = 0;
    let mut r = n - 1;
    for _ in 0..20 {
        let m = l + (r - l) / 2;
        println!("{}", m);
        std::io::stdout().flush().ok();
        let s: String = read(&mut stdin_lock, &mut buf, b'\n');
        let s_m = is_male(s.as_str());
        if s_m.is_none() {
            return;
        }
        if (m % 2 == 0 && s_m == s_0) || (m % 2 != 0 && s_m == s_n1) {
            l = m;
        } else {
            r = m;
        }
    }
}
