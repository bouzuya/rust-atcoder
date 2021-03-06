use std::collections::HashMap;

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
    let sv: Vec<char> = s.chars().collect();
    let tv: Vec<char> = t.chars().collect();

    let mut ans = true;
    let mut sm = HashMap::new();
    let mut tm = HashMap::new();
    for i in 0..sv.len() {
        if sm.get(&sv[i]) != tm.get(&tv[i]) {
            ans = false;
            break;
        }
        if !sm.contains_key(&sv[i]) {
            let c = sm.len();
            sm.insert(sv[i], c);
            tm.insert(tv[i], c);
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
